use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub fn move_files(
    root_path: &str,
    files: &HashMap<String, Vec<String>>
) {
    if let Some(items) = files.get("archive") {
        create_folder_and_move_files(&format!("{}/archive", root_path), items);
    }
    if let Some(items) = files.get("document") {
        create_folder_and_move_files(&format!("{}/document", root_path), items);
    }
    if let Some(items) = files.get("image") {
        create_folder_and_move_files(&format!("{}/image", root_path), items);
    }
    if let Some(items) = files.get("video") {
        create_folder_and_move_files(&format!("{}/video", root_path), items);
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

