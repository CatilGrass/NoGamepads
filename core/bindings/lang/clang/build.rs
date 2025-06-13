fn main() {
    // Get the project root directory
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let root_path = std::path::Path::new(&manifest_dir);

    // Input configuration directory path
    let config_dir = root_path.join("configures");

    // Output directory path (headers folder in the crate directory)
    let headers_dir = root_path.join("headers");

    // Clear existing headers directory (if it exists)
    if headers_dir.exists() {
        std::fs::remove_dir_all(&headers_dir).expect("Failed to remove headers directory");
    }

    // Create a new headers directory
    std::fs::create_dir_all(&headers_dir).expect("Failed to create headers directory");

    // Find all TOML files in configures directory
    let config_files = std::fs::read_dir(&config_dir)
        .expect("Failed to read configures directory")
        .filter_map(|entry| {
            let entry = entry.expect("Failed to read directory entry");
            let path = entry.path();
            if path.is_file() && path.extension().map_or(false, |ext| ext == "toml") {
                Some(path)
            } else {
                None
            }
        });

    // Process each configuration file
    for config_path in config_files {
        // Extract filename without extension
        let file_stem = config_path.file_stem()
            .and_then(|s| s.to_str())
            .expect("Invalid filename")
            .to_string();

        // Set output file path
        let output_file = headers_dir.join(format!("{}.h", file_stem));

        // Load configuration from current TOML file
        let config = cbindgen::Config::from_file(&config_path)
            .expect(&format!("Failed to parse configuration: {}", config_path.display()));

        // Generate header file
        cbindgen::Builder::new()
            .with_crate(&manifest_dir)
            .with_config(config)
            .generate()
            .expect(&format!("Failed to generate header for: {}", config_path.display()))
            .write_to_file(&output_file);
    }

    // Print rerun instructions
    println!("cargo:rerun-if-changed={}", config_dir.display());
    println!("cargo:rerun-if-changed=src/mycode.rs");
    println!("cargo:rerun-if-changed=build.rs");
}