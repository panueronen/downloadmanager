use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub folder_to_watch: String,
    pub file_rules: Vec<FileRule>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FileRule {
    pub extension: String,
    pub folder: String,
}

pub fn load_config(file_path: &Path) -> Config {
    let config_content =
        fs::read_to_string(file_path).expect("Failed to read the configuration file.");
    toml::from_str(&config_content).expect("Failed to parse the configuration file.")
}

pub fn save_config(file_path: &Path, config: &Config) {
    let config_content = toml::to_string(config).expect("Failed to serialize the configuration.");
    fs::write(file_path, config_content).expect("Failed to write the configuration file.");
}

pub fn default_config() -> Config {
    Config {
        folder_to_watch: String::from("C:\\Users\\panue\\Downloads"),
        file_rules: vec![
            FileRule {
                extension: String::from("jpg"),
                folder: String::from("Images"),
            },
            FileRule {
                extension: String::from("pdf"),
                folder: String::from("Documents"),
            },
            FileRule {
                extension: String::from("zip"),
                folder: String::from("Archives"),
            },
        ],
    }
}
