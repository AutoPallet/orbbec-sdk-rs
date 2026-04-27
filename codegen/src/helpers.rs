pub(crate) fn compute_trimmed_names(variants: &[String]) -> Vec<String> {
    let split: Vec<Vec<&str>> = variants
        .iter()
        .map(|v| v.split('_').filter(|s| !s.is_empty()).collect())
        .collect();

    let common_len = longest_common_prefix_len(&split);

    let mut strip_len = common_len;

    while strip_len > 0 {
        let ok = split.iter().all(|parts| {
            let remaining = &parts[strip_len..];
            match remaining.first() {
                Some(first) => starts_like_rust_ident(first),
                None => false,
            }
        });

        if ok {
            break;
        }

        strip_len -= 1;
    }

    split
        .iter()
        .map(|parts| {
            let remaining = if strip_len < parts.len() {
                &parts[strip_len..]
            } else {
                &parts[parts.len() - 1..]
            };

            remaining.join("_")
        })
        .collect()
}

fn longest_common_prefix_len(parts: &[Vec<&str>]) -> usize {
    if parts.is_empty() {
        return 0;
    }

    let min_len = parts.iter().map(Vec::len).min().unwrap_or(0);
    let mut i = 0;

    while i < min_len {
        let tok = parts[0][i];
        if parts.iter().all(|p| p[i] == tok) {
            i += 1;
        } else {
            break;
        }
    }

    i
}

fn starts_like_rust_ident(s: &str) -> bool {
    let mut chars = s.chars();
    match chars.next() {
        Some(c) => c == '_' || c.is_ascii_alphabetic(),
        None => false,
    }
}

pub(crate) fn doc_strings(attrs: &[syn::Attribute]) -> Vec<String> {
    attrs
        .iter()
        .filter_map(|attr| {
            if !attr.path().is_ident("doc") {
                return None;
            }

            let syn::Meta::NameValue(meta) = &attr.meta else {
                return None;
            };

            let syn::Expr::Lit(expr_lit) = &meta.value else {
                return None;
            };

            let syn::Lit::Str(lit) = &expr_lit.lit else {
                return None;
            };

            Some(lit.value())
        })
        .collect()
}

pub(crate) fn pretty_print_file(items: proc_macro2::TokenStream) -> String {
    let file = syn::parse2::<syn::File>(items)
        .inspect_err(|e| {
            eprintln!("Error parsing item: {e:?}");
        })
        .unwrap();
    prettyplease::unparse(&file)
}

/// Run `rustfmt --edition 2024` against a generated file in place. The
/// project is on edition 2024, so anything bindgen / prettyplease emits
/// must be reformatted with the matching edition or `cargo fmt --check`
/// in CI flags it as drift.
pub(crate) fn rustfmt_in_place(path: &std::path::Path) {
    let status = std::process::Command::new("rustfmt")
        .args(["--edition", "2024"])
        .arg(path)
        .status()
        .expect("failed to run rustfmt — is it installed in the active toolchain?");
    if !status.success() {
        panic!("rustfmt failed for {}", path.display());
    }
}
