use colored::Colorize;
use std::collections::HashMap;
use std::{fs, io};
use std::path::{Path, PathBuf};
use tabled::{
    settings::Style, Table,
    Tabled,
};
use thiserror::Error;

#[derive(Error, Debug)]
enum FileError {
    #[error("Failed to create folder '{path}': {source}")]
    FolderCreation {
        path: PathBuf,
        source: io::Error,
    },

    #[error("File '{path}' cannot be moved : {source}")]
    FileMove {
        path: PathBuf,
        source: io::Error,
    },

    #[error("The destination '{path}' already exists.")]
    DestinationExists {
        path: PathBuf,
    },

    #[error("No permission to access '{path}'")]
    PermissionDenied {
        path: PathBuf,
    },

    #[error("File '{path}' not found")]
    NotFound {
        path: PathBuf,
    },
}

#[derive(Tabled, Debug)]
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
           preview_file_moves(files, root_path, destination, dry_run);
       }
       false => {
           preview_file_moves(files, root_path, destination, dry_run);
           if let Some(items) = files.get("archive") {
               create_folder_and_move_files(&format!("{}/archive", destination), items, root_path);
           }
           if let Some(items) = files.get("document") {
               create_folder_and_move_files(&format!("{}/document", destination), items, root_path);
           }
           if let Some(items) = files.get("image") {
               create_folder_and_move_files(&format!("{}/image", destination), items, root_path);
           }
           if let Some(items) = files.get("video") {
               create_folder_and_move_files(&format!("{}/video", destination), items, root_path);
           }
       }
   }
}

fn preview_file_moves(files: &HashMap<String, Vec<String>>, path_scan: &str, destination: &str, dry_run: bool) {
    let mut file_moves = Vec::new();
    for (key, value) in files {
        for item in value {
            file_moves.push(FileMove{
                before: format!("{}/{}", path_scan, item),
                after: format!("{}/{}", destination, item),
            })
        }
    }

    let mut table = Table::new(file_moves);
    table.with(Style::modern());

    println!("Scanning folder : {}", path_scan);
    println!("Destination folder : {}", destination);
    println!("Dry run: {}", dry_run);
    println!("Preview file moves:");
    if dry_run {
        println!("{}", "Dry-run mode: no files will be moved.".red().bold());
    }
    println!("{}", table);
}

fn create_folder_and_move_files(dir: &str, file_paths: &[String], src_path: &str ) -> Result<(), FileError> {
    let destination = Path::new(dir);

    if !destination.exists() {
        fs::create_dir_all(destination)
            .map_err(|e| match e.kind() {
                io::ErrorKind::PermissionDenied => FileError::PermissionDenied {
                    path: destination.to_path_buf(),
                },
                _ => FileError::FolderCreation {
                    path: destination.to_path_buf(),
                    source: e,
                },
            })?;
    }


    for file in file_paths {
        let src = Path::new(src_path).join(file);

        if !src.exists() {
            eprintln!("{}", FileError::NotFound { path: src.to_path_buf() });
            continue;
        }

        let file_name = match src.file_name() {
            Some(f) => f,
            None => {
                eprintln!("{} : invalid filename", file.red());
                continue;
            }
        };

        let new_path = destination.join(file_name);

        if new_path.exists() {
            eprintln!("{}" , FileError::DestinationExists { path: new_path.clone()});
        }

        if let Err(e) = fs::rename(&src, &new_path) {
            match e.kind() {
                io::ErrorKind::PermissionDenied => {
                    eprintln!("{}", FileError::PermissionDenied { path: src.to_path_buf() });
                }
                _ => {
                    eprintln!("{}", FileError::FileMove {
                        path: src.to_path_buf(),
                        source: e,
                    });
                }
            }
            continue;
        }
    }

    Ok(())
}

