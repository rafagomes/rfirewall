use dirs_next::config_dir;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::PathBuf;

pub fn ensure_config_dir_exists() {
    let config_path = get_config_path();

    if !config_path.exists() {
        fs::create_dir_all(&config_path).expect("Failed to create configuration directory");
    }
}

fn get_config_path() -> PathBuf {
    let mut config_path = config_dir().expect("Could not find config directory");

    config_path.push("rust_firewall");

    config_path
}

pub fn read_config_file(file_name: &str) -> String {
    let mut file_path = get_config_path();
    file_path.push(file_name);

    let mut file = File::open(&file_path).expect("Failed to open config file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read config file");

    contents
}

pub fn write_config_file(file_name: &str, contents: &str) {
    let mut file_path = get_config_path();
    file_path.push(file_name);

    let mut file = File::create(&file_path).expect("Failed to create config file");
    file.write_all(contents.as_bytes())
        .expect("Failed to write config file");
}
