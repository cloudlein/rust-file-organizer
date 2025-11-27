use std::fs;
use colored::Colorize;
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
    let raw = fs::read_to_string("config.toml").unwrap_or_else(|_| {
        eprintln!("{}", "Error: Unable to read config.toml".red().bold());
        eprintln!("Make sure config.toml exists in the current directory.");
        std::process::exit(1);
    });

    toml::from_str(&raw).unwrap_or_else(|_| {
        eprintln!("{}", "Error: Failed to parse config.toml".red().bold());
        eprintln!("Check for invalid keys or syntax.");
        std::process::exit(1);
    })
}
