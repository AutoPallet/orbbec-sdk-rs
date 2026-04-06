use std::path::{Path, PathBuf};
use std::{collections::HashMap, sync::Arc};

use convert_case::Casing;
use proc_macro2::{Ident, Span};
use regex::Regex;

use crate::custom::{VARIANT_RENAMES, struct_value_type};
use crate::helpers::{compute_trimmed_names, doc_strings, pretty_print_file};

mod custom;
mod helpers;

pub struct GenerateArgs {
    pub work_source_dir: PathBuf,
    pub bindings_path: PathBuf,
    pub target: String,
}

fn get_builder(work_source_dir: &Path, target: &str) -> bindgen::Builder {
    let include_path = work_source_dir.join("include");
    bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header(
            include_path
                .join("libobsensor/ObSensor.h")
                .to_str()
                .unwrap(),
        )
        // Set the include path for the OrbbecSDK headers
        .clang_arg(format!("-I{}", include_path.display()))
        // Set the target architecture (The committed bindings are for x86_64-unknown-linux-gnu!)
        .clang_arg(format!("--target={}", target))
        // Convert enum type
        .translate_enum_integer_types(true)
        .default_enum_style(bindgen::EnumVariation::Rust {
            non_exhaustive: true,
        })
}

pub fn generate_bindings(args: GenerateArgs) {
    // first pass, we don't emit anything, we just collect the data we need to generate the bindings
    let enum_data = Arc::new(std::sync::Mutex::new(EnumData::new()));
    let enum_collector = EnumCollector::new(enum_data.clone());

    get_builder(&args.work_source_dir, &args.target)
        .parse_callbacks(Box::new(enum_collector))
        .generate()
        .expect("Unable to generate bindings, first pass");

    let (enum_case_renames, property_id_types) = enum_data.lock().unwrap().compute_enum_cases();

    let bindings = {
        let enum_data = enum_data.lock().unwrap();
        get_builder(&args.work_source_dir, &args.target)
            // Tell cargo to invalidate the built crate whenever any of the
            // included header files changed.
            .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
            // Rename everything to make it more Rust-y
            .parse_callbacks(Box::new(Renamer::new(
                enum_data.compute_type_renames(),
                enum_case_renames,
            )))
            // Finish the builder and generate the bindings.
            .generate()
            // Unwrap the Result and panic on failure.
            .expect("Unable to generate bindings, main pass")
    };

    // Write the bindings to file
    std::fs::create_dir_all(&args.bindings_path).expect("Failed to create bindings directory");
    bindings
        .write_to_file(args.bindings_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    let struct_bindings = get_builder(&args.work_source_dir, &args.target)
        .header("OrbbecSDK/src/shared/InternalTypes.hpp")
        .allowlist_type("OBDeviceTime")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate struct bindings");

    struct_bindings
        .write_to_file(args.bindings_path.join("structs.rs"))
        .expect("Couldn't write struct bindings!");

    // Now, generate the property_id_types.rs file
    let property_id_types_file = args.bindings_path.join("property_id_types.rs");
    {
        // Also parse the bindings to get the OBPropertyID enum comments
        let parsed_file = syn::parse_file(&bindings.to_string()).expect("Failed to parse bindings");

        let property_id_enum = parsed_file
            .items
            .iter()
            .find_map(|item| match item {
                syn::Item::Enum(item) if item.ident == "OBPropertyID" => Some(item),
                _ => None,
            })
            .expect("Failed to find OBPropertyID enum");

        let mut doc_comments = HashMap::new();

        for variant in &property_id_enum.variants {
            let docs = doc_strings(&variant.attrs);
            let doc_comment = docs.join("\n");
            doc_comments.insert(variant.ident.to_string(), doc_comment);
        }

        let tokens = generate_property_id_types_file(&property_id_types, &doc_comments);
        std::fs::write(property_id_types_file, pretty_print_file(tokens))
            .expect("Failed to write property_id_types file");
    }
}

fn generate_property_id_types_file(
    property_id_types: &HashMap<String, PropertyIDType>,
    doc_comments: &HashMap<String, String>,
) -> proc_macro2::TokenStream {
    const NO_DOC_COMMENT: &str = "No doc comment found";
    let mut tokens = proc_macro2::TokenStream::new();
    let mut property_id_types = property_id_types.iter().collect::<Vec<_>>();
    property_id_types.sort_by_key(|(name, _)| *name);

    for (name, prop_type) in property_id_types {
        let name_token = Ident::new(name, Span::call_site());
        let doc_comment = doc_comments
            .get(name)
            .map(|s| s.as_str())
            .unwrap_or(NO_DOC_COMMENT);
        let prop_tokens = match prop_type {
            PropertyIDType::Bool => quote::quote! {
                define_bool_property!(#name_token, #doc_comment);
            },
            PropertyIDType::Int => quote::quote! {
                define_int_property!(#name_token, #doc_comment);
            },
            PropertyIDType::Float => quote::quote! {
                define_float_property!(#name_token, #doc_comment);
            },
            PropertyIDType::Struct => {
                let ty = struct_value_type(name.as_str());
                if let Some(ty) = ty {
                    quote::quote! {
                        define_struct_property!(#name_token, #ty, #doc_comment);
                    }
                } else {
                    // If we don't have a struct value type, we can't generate the property
                    tokens.extend(quote::quote! {
                        #[doc = "This property is a struct, but we don't have a struct value type"]
                        struct #name_token;
                    });
                    continue;
                }
            }
            PropertyIDType::RawData => continue, // TODO: handle RawData
        };

        tokens.extend(prop_tokens);
    }
    tokens
}

#[derive(Debug)]
struct EnumData {
    enum_cases: HashMap<String, Vec<String>>,
    type_names: HashMap<bindgen::callbacks::DiscoveredItemId, Vec<String>>,
    variant_renames: HashMap<&'static str, &'static str>,
}

type EnumCaseRenames = HashMap<String, HashMap<String, String>>;
type PropertyIDTypes = HashMap<String, PropertyIDType>;

impl EnumData {
    fn new() -> Self {
        Self {
            enum_cases: HashMap::new(),
            type_names: HashMap::new(),
            variant_renames: HashMap::from_iter(VARIANT_RENAMES.iter().copied()),
        }
    }

    fn compute_enum_cases(&self) -> (EnumCaseRenames, PropertyIDTypes) {
        let mut enum_case_renames: EnumCaseRenames = HashMap::new();
        let mut property_id_types = HashMap::new();
        for (enum_name, variants) in self.enum_cases.iter() {
            let renamed_variants = variants
                .iter()
                .map(|v| {
                    self.variant_renames
                        .get(v.as_str())
                        .unwrap_or(&v.as_str())
                        .to_string()
                })
                .collect::<Vec<_>>();

            let entry = enum_case_renames.entry(enum_name.clone()).or_default();

            if enum_name == "OBPropertyID" {
                for (original, renamed) in variants.iter().zip(renamed_variants.iter()) {
                    let (prop_type, variant_name) = self.map_property_id(renamed);
                    property_id_types.insert(variant_name.clone(), prop_type);
                    entry.insert(original.clone(), variant_name.clone());
                }
            } else {
                for (original, trimmed) in variants
                    .iter()
                    .zip(compute_trimmed_names(&renamed_variants))
                {
                    entry.insert(
                        original.clone(),
                        trimmed.to_case(convert_case::Case::Pascal),
                    );
                }
            }
        }
        (enum_case_renames, property_id_types)
    }

    fn compute_type_renames(&self) -> HashMap<String, Option<String>> {
        let mut type_renames = HashMap::new();
        // regex to match OB<capital_letter>... since we want to keep these as is
        // we don't use casing here because it would convert OB to Ob.
        let re = Regex::new(r"^OB([A-Z])").unwrap();
        for (_item_id, type_names) in self.type_names.iter() {
            let first_name = type_names.first().expect("should never be empty");

            for type_name in type_names {
                if re.is_match(type_name) {
                    // Indicate that we should rename instances of the canonical name to the matched name
                    type_renames.insert(first_name.clone(), Some(type_name.clone()));
                } else {
                    // Indicate that we should drop this type name entirely
                    type_renames.insert(type_name.clone(), None);
                }
            }
        }

        type_renames
    }

    fn map_property_id(&self, name: &str) -> (PropertyIDType, String) {
        let (prop_type, name) = match name {
            name if name.ends_with("_BOOL") => {
                (PropertyIDType::Bool, name.strip_suffix("_BOOL").unwrap())
            }
            name if name.ends_with("_INT") => {
                (PropertyIDType::Int, name.strip_suffix("_INT").unwrap())
            }
            name if name.ends_with("_FLOAT") => {
                (PropertyIDType::Float, name.strip_suffix("_FLOAT").unwrap())
            }
            name if name.starts_with("OB_STRUCT_") => (
                PropertyIDType::Struct,
                name.strip_prefix("OB_STRUCT_").unwrap(),
            ),
            name if name.starts_with("OB_RAW_DATA_") => (
                PropertyIDType::RawData,
                name.strip_prefix("OB_RAW_DATA_").unwrap(),
            ),
            name => todo!("Unknown property type: {}", name),
        };

        let name = match name {
            name if name.starts_with("OB_PROP_") => name.strip_prefix("OB_PROP_").unwrap(),
            name if name.starts_with("OB_DEVICE_") => name.strip_prefix("OB_DEVICE_").unwrap(),
            name => name,
        };

        (prop_type, name.to_case(convert_case::Case::Pascal))
    }
}

#[derive(Debug)]
struct EnumCollector {
    data: Arc<std::sync::Mutex<EnumData>>,
}

impl EnumCollector {
    fn new(data: Arc<std::sync::Mutex<EnumData>>) -> Self {
        Self { data }
    }
}

impl bindgen::callbacks::ParseCallbacks for EnumCollector {
    fn new_item_found(
        &self,
        id: bindgen::callbacks::DiscoveredItemId,
        item: bindgen::callbacks::DiscoveredItem,
        _source_loc: Option<&bindgen::callbacks::SourceLocation>,
    ) {
        // Record aliases to enums so we can pick the one with the right casing
        match item {
            bindgen::callbacks::DiscoveredItem::Enum { final_name } => {
                self.data
                    .lock()
                    .unwrap()
                    .type_names
                    .insert(id, vec![final_name]);
            }
            bindgen::callbacks::DiscoveredItem::Alias {
                alias_name,
                alias_for,
            } => {
                let mut data = self.data.lock().unwrap();
                if let Some(aliases) = data.type_names.get_mut(&alias_for) {
                    aliases.push(alias_name);
                }
            }
            _ => (),
        }
    }

    fn enum_variant_name(
        &self,
        enum_name: Option<&str>,
        variant_name: &str,
        _variant_value: bindgen::callbacks::EnumVariantValue,
    ) -> Option<String> {
        let enum_name = enum_name?;

        let mut data = self.data.lock().unwrap();
        data.enum_cases
            .entry(enum_name.to_string())
            .or_insert(vec![])
            .push(variant_name.to_string());

        None
    }
}

#[derive(Debug, Clone, Copy)]
enum PropertyIDType {
    Bool,
    Int,
    Float,
    Struct,
    RawData,
}

#[derive(Debug)]
struct Renamer {
    type_renames: HashMap<String, Option<String>>,
    enum_case_renames: HashMap<String, HashMap<String, String>>,
}

impl Renamer {
    fn new(
        type_renames: HashMap<String, Option<String>>,
        enum_case_renames: HashMap<String, HashMap<String, String>>,
    ) -> Self {
        Self {
            type_renames,
            enum_case_renames,
        }
    }
}

impl bindgen::callbacks::ParseCallbacks for Renamer {
    fn item_name(&self, item_info: bindgen::callbacks::ItemInfo) -> Option<String> {
        if let Some(type_rename) = self.type_renames.get(item_info.name) {
            return type_rename.clone();
        }
        None
    }

    fn field_name(&self, field_info: bindgen::callbacks::FieldInfo) -> Option<String> {
        Some(
            field_info
                .field_name
                .to_case(convert_case::Case::Snake)
                .to_string(),
        )
    }

    fn process_comment(&self, comment: &str) -> Option<String> {
        // Lots of the comments start with <, so we strip that off to avoid confusion
        let comment = comment.strip_prefix("< ").unwrap_or(comment);
        // Trim whitespace
        Some(comment.trim().to_string())
    }
    fn enum_variant_behavior(
        &self,
        _enum_name: Option<&str>,
        original_variant_name: &str,
        _variant_value: bindgen::callbacks::EnumVariantValue,
    ) -> Option<bindgen::callbacks::EnumVariantCustomBehavior> {
        // Hide _TYPE_COUNT variants
        if original_variant_name.ends_with("_TYPE_COUNT") {
            return Some(bindgen::callbacks::EnumVariantCustomBehavior::Hide);
        }
        None
    }

    fn enum_variant_name(
        &self,
        enum_name: Option<&str>,
        original_variant_name: &str,
        _variant_value: bindgen::callbacks::EnumVariantValue,
    ) -> Option<String> {
        self.enum_case_renames
            .get(enum_name?)
            .and_then(|cases| cases.get(original_variant_name).cloned())
    }
}
