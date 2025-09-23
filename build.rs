fn main() {
    if cfg!(feature = "docs-only") {
        return;
    }

    // Configure and build the C++ library using CMake
    let mut dst = cmake::Config::new("OrbbecSDK");
    dst.define("CMAKE_POLICY_VERSION_MINIMUM", "3.10");
    dst.define("OB_BUILD_DOCS", "OFF");
    dst.define("OB_BUILD_TOOLS", "OFF");
    dst.define("OB_BUILD_TESTS", "OFF");

    // cmake-rs has some issues setting the correct flags on Windows
    #[cfg(target_os = "windows")]
    dst.static_crt(true)
        .cflag("/DWIN32 /D_WINDOWS -w")
        .cxxflag("/DWIN32 /D_WINDOWS -w /EHsc");

    // Disable warnings
    #[cfg(not(target_os = "windows"))]
    dst.cflag("-w").cxxflag("-w");

    let dst = dst.build();

    // Binary directory differs between Windows and Unix
    #[cfg(target_os = "windows")]
    let bin_folder = "bin";

    #[cfg(not(target_os = "windows"))]
    let bin_folder = "lib";

    let bin_dir = dst.join(bin_folder);
    let target_dir = dst.join("../../../");

    // Copy extension directory to target_dir
    let extensions_dir = bin_dir.join("extensions");
    let dest_extensions_dir = target_dir.join("extensions");
    if !dest_extensions_dir.exists() {
        std::fs::create_dir_all(&dest_extensions_dir)
            .expect("Failed to create extensions directory");
    }

    // Go through each folder in extensions_dir and copy to dest_extensions_dir
    for entry in std::fs::read_dir(&extensions_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            let folder_name = path.file_name().unwrap();
            let dest_folder = dest_extensions_dir.join(folder_name);
            if !dest_folder.exists() {
                std::fs::create_dir_all(&dest_folder)
                    .expect("Failed to create extension subdirectory");
            }

            for file_entry in std::fs::read_dir(&path).unwrap() {
                let file_entry = file_entry.unwrap();
                let file_path = file_entry.path();
                if let Some(ext) = file_path.extension()
                    && (ext == "dll" || ext == "so" || ext == "dylib")
                {
                    let file_name = file_path.file_name().unwrap();
                    let dest_file = dest_folder.join(file_name);
                    std::fs::copy(&file_path, &dest_file)
                        .expect("Failed to copy extension library file");
                }
            }
        }
    }

    // Copy libraries to target directory
    for entry in std::fs::read_dir(&bin_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if let Some(ext) = path.extension()
            && (ext == "dll" || ext == "so" || ext == "dylib")
        {
            let file_name = path.file_name().unwrap();
            let dest = target_dir.join(file_name);
            std::fs::copy(&path, &dest).expect("Failed to copy library file");
        }
    }

    // Tell cargo to tell rustc to link the OrbbecSDK library
    println!(
        "cargo:rustc-link-search=native={}",
        dst.join("lib").display()
    );
    println!("cargo:rustc-link-lib=OrbbecSDK");

    // Generate bindings using bindgen
    #[cfg(feature = "buildtime-bindgen")]
    {
        let cargo_manifest_dir = std::env::current_dir().unwrap();

        // The bindgen::Builder is the main entry point
        // to bindgen, and lets you build up options for
        // the resulting bindings.
        let bindings = bindgen::Builder::default()
            // The input header we would like to generate
            // bindings for.
            .header("OrbbecSDK/include/libobsensor/ObSensor.h")
            // Tell cargo to invalidate the built crate whenever any of the
            // included header files changed.
            .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
            // Set the include path for the OrbbecSDK headers
            .clang_arg("-IOrbbecSDK/include/")
            // Convert enum type
            .translate_enum_integer_types(true)
            // Finish the builder and generate the bindings.
            .generate()
            // Unwrap the Result and panic on failure.
            .expect("Unable to generate bindings");

        // Write the bindings to file
        let bindings_dir = cargo_manifest_dir.join("bindings");
        let bindings_file = bindings_dir.join("bindings.rs");

        std::fs::create_dir_all(&bindings_dir).expect("Failed to create bindings directory");

        bindings
            .write_to_file(bindings_file)
            .expect("Couldn't write bindings!");
    }
}
