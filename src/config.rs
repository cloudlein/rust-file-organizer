use std::fs;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub folders: Folders
}

#[derive(Debug, Deserialize)]
pub struct Folders {
    pub images: Vec<String>,
    pub documents: Vec<String>,
    pub archives: Vec<String>,
    pub videos: Vec<String>
}
pub fn load_config() ->  Config {
    // println!("Config module loaded (placeholder).");
    let raw = fs::read_to_string("config.toml")
        .expect("Failed to read config.toml");

    toml::from_str(&raw)
        .expect("Failed to parse config.toml")

}
