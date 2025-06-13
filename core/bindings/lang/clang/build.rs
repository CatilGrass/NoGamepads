fn main() {

    // Get the project root directory
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let root_path = std::path::Path::new(&manifest_dir);

    // Input configuration file path
    let config_path = root_path.join("Generate.toml");

    // Output directory path (headers folder in the crate directory)
    let headers_dir = root_path.join("headers");

    // Create the headers directory (if it does not exist)
    std::fs::create_dir_all(&headers_dir).expect("Failed to create headers directory");

    // Output file path
    let output_file = headers_dir.join("nogamepads.h");

    // Load configuration from Generate.toml
    let config = cbindgen::Config::from_file(&config_path)
        .expect("Failed to parse Generate.toml configuration");

    // Generate header file
    cbindgen::Builder::new()
        .with_crate(manifest_dir)
        .with_config(config)
        .generate()
        .expect("Failed to generate C header file")
        .write_to_file(&output_file);

    println!("cargo:rerun-if-changed={}", config_path.display());
    println!("cargo:rerun-if-changed=src/mycode.rs");
    println!("cargo:rerun-if-changed=build.rs");
}