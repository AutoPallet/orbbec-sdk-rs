use std::path::PathBuf;

fn main() {
    // change directory to the root of the project
    std::env::set_current_dir("..").unwrap();
    let cargo_manifest_dir = std::env::current_dir().unwrap();
    // TODO(danny): This is obnoxious for testing. Figure out a better way.
    // You must provide the latest contents of the built version of the
    // OrbbecSDK submodule (build.rs's work_src variable contents)
    // in the environment variable WORK_DIR.
    let work_source_dir = std::env::var("WORK_DIR")
        .map(PathBuf::from)
        .unwrap_or(cargo_manifest_dir.join("OrbbecSDK"));
    let sys_path = cargo_manifest_dir.join("src/sys");

    codegen::generate_bindings(codegen::GenerateArgs {
        work_source_dir,
        sys_path,
        target: std::env::var("TARGET").unwrap_or("x86_64-unknown-linux-gnu".to_string()),
    });
}
