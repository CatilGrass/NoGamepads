use cbindgen::{generate_with_config, Config, Language, ParseConfig, ParseExpandConfig};
use std::env;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    let config_c = Config {
        language: Language::C,
        header: Some("/* NoGamepads C Bindings. */".to_string()),

        parse: ParseConfig {
            expand: ParseExpandConfig {
                all_features: true,
                ..Default::default()
            },
            ..Default::default()
        },
        ..Config::default()
    };

    let mut config_cpp = config_c.clone();
    config_cpp.language = Language::Cxx;
    config_cpp.include_version = true;
    config_cpp.header = Some("/* NoGamepads C++ Bindings. */".to_string());
    config_cpp.namespace = Some("nogamepads".to_string());

    generate_with_config(&crate_dir, config_c)
        .expect("Could not generate nogamepads.h")
        .write_to_file("../.cargo/shared/target/release/deps/nogamepads.h");
    generate_with_config(&crate_dir, config_cpp)
        .expect("Could not generate nogamepads_cpp.h")
        .write_to_file("../.cargo/shared/target/release/deps/nogamepads_cpp.h");
}