use std::collections::HashMap;
use std::fs;
use std::path::Path;
use colored::Colorize;
use tabled::Tabled;

#[derive(Tabled)]
struct FileMove {

    #[tabled(rename = "Before")]
    before: String,

    #[tabled(rename = "After")]
    after: String,

}

pub fn move_files(
    root_path: &str,
    destination: &str,
    files: &HashMap<String, Vec<String>>,
    dry_run: bool
) {
   match dry_run {
       true => {
           let mut file_moves: Vec<String> = Vec::new();
           for (key, value) in files {
              FileMove{
                  before: 
              }
           }
       }
       false => {
           if let Some(items) = files.get("archive") {
               create_folder_and_move_files(&format!("{}/archive", destination), items);
           }
           if let Some(items) = files.get("document") {
               create_folder_and_move_files(&format!("{}/document", destination), items);
           }
           if let Some(items) = files.get("image") {
               create_folder_and_move_files(&format!("{}/image", destination), items);
           }
           if let Some(items) = files.get("video") {
               create_folder_and_move_files(&format!("{}/video", destination), items);
           }
       }
   }
}

fn create_folder_and_move_files(dir: &str, file_paths: &[String]) {
    let destination = Path::new(dir);

    fs::create_dir_all(destination).unwrap();

    for file in file_paths {
        let src = Path::new(file);
        let file_name = src.file_name().unwrap();
        let new_path = destination.join(file_name);

        fs::rename(src, new_path).unwrap();
    }
}

