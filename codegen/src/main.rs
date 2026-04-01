fn main() {
    // change directory to the root of the project
    std::env::set_current_dir("..").unwrap();
    let cargo_manifest_dir = std::env::current_dir().unwrap();
    let bindings_path = cargo_manifest_dir.join("bindings");

    codegen::generate_properties(codegen::GenerateArgs {
        bindings_path,
        target: std::env::var("TARGET").unwrap_or("x86_64-unknown-linux-gnu".to_string()),
    });
}
