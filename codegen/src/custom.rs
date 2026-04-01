use proc_macro2::Span;
use syn::Ident;

pub(crate) const VARIANT_RENAMES: &[(&str, &str)] = &[
    ("OB_HOLE_FILL_FAREST", "OB_HOLE_FILL_FARTHEST"),
    // All the other types have the common prefix of OB_EXCEPTION_TYPE_
    (
        "OB_EXCEPTION_STD_EXCEPTION",
        "OB_EXCEPTION_TYPE_STD_EXCEPTION",
    ),
];

pub(crate) fn struct_value_type(name: &str) -> Option<Ident> {
    match name {
        "DeviceTime" => Some(Ident::new("OBDeviceTime", Span::call_site())),
        _ => None,
    }
}
