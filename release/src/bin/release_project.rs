use std::fs::{copy, create_dir_all, remove_dir_all};
use std::path::PathBuf;

use serde::Deserialize;
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub release: Release,
}

#[derive(Debug, Deserialize)]
pub struct Release {
    pub root: HashMap<String, ReleaseItem>,
    pub deps: HashMap<String, ReleaseItem>,
}

#[derive(Debug, Deserialize)]
pub struct ReleaseItem {
    pub raw: String,
    pub target: String,
    pub files: Vec<String>,
}

pub fn main() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).parent().unwrap().to_path_buf();

    let toml_config = root.join("Release.toml");
    let config: Config = toml::from_str(fs::read_to_string(toml_config).unwrap().as_str())
        .expect("Failed to parse TOML");

    let release_root =
        root.join("release");

    let release_dir_current_version =
        release_root.join(env!("PROJECT_VERSION"));

    let target_dir =
        root.join(".cargo").join("shared").join("target").join("release").join("deps");

    // 清理目录
    let _ = remove_dir_all(&release_dir_current_version);

    for data in config.release.root {
        println!("Releasing \"{}\" files", data.0);
        let raw_path = root.join(data.1.raw);
        let target_path = release_dir_current_version.join(data.1.target);
        copy_files(&raw_path, &target_path, data.1.files);
    }

    for data in config.release.deps {
        println!("Releasing \"{}\" files", data.0);
        let raw_path = target_dir.join(data.1.raw);
        let target_path = release_dir_current_version.join(data.1.target);
        copy_files(&raw_path, &target_path, data.1.files);
    }
}

fn copy_files (raw: &PathBuf, target: &PathBuf, file_names: Vec<String>) {
    create_dir_all(target).unwrap();

    for file in file_names.iter() {
        let file_path_raw = raw.join(file);
        let file_path_target = target.join(file);
        if file_path_raw.exists() {
            let _ = copy(&file_path_raw, &file_path_target);
        }
    }
}