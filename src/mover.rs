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
pub enum FileError {
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

pub fn move_files (
    root_path: &str,
    destination: &str,
    files: &HashMap<String, Vec<String>>,
    dry_run: bool
) -> Result<(), FileError> {
    preview_file_moves(files, root_path, destination, dry_run);
    if dry_run {
        return Ok(());
    }

    preview_file_moves(files, root_path, destination, dry_run);

    for (category, items) in files {
        let folder = format!("{}/{}", destination, category);
        create_folder_and_move_files(&folder, items, root_path)
            .expect("Cant create folder and move items");
    }

    Ok(())
}

fn preview_file_moves(
    files: &HashMap<String, Vec<String>>,
    path_scan: &str,
    destination: &str,
    dry_run: bool
) {
    let mut file_moves = Vec::new();
    for (category, items) in files {
        for item in items {
            file_moves.push(FileMove{
                before: format!("{}/{}", path_scan, item),
                after: format!("{}/{}/{}", destination, category, item),
            })
        }
    }

    let mut table = Table::new(file_moves);
    table.with(Style::modern());

    println!("Scanning folder : {}", path_scan);
    println!("Destination folder : {}", destination);
    println!("Dry run: {}", dry_run.to_string().green().bold());
    println!("Preview file moves:");
    if dry_run {
        println!("{}", "Dry-run mode: no files will be moved.".red().bold());
    }
    println!("{}", table);
}

fn create_folder_and_move_files(
    dir: &str,
    file_paths: &[String],
    src_path: &str,
) -> Result<(), FileError> {

    let destination = Path::new(dir);

    if !destination.exists() {
        fs::create_dir_all(destination).map_err(|e| match e.kind() {
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
            eprintln!("{}", FileError::NotFound {
                path: src.to_path_buf(),
            });
            continue;
        }

        let file_name = src.file_name().unwrap();
        let new_path = destination.join(file_name);

        if new_path.exists() {
            eprintln!("{}", FileError::DestinationExists {
                path: new_path.clone(),
            });
            continue;
        }

        fs::rename(&src, &new_path).map_err(|e| match e.kind() {
            io::ErrorKind::PermissionDenied => FileError::PermissionDenied {
                path: src.to_path_buf(),
            },
            _ => FileError::FileMove {
                path: src.to_path_buf(),
                source: e,
            },
        })?;
    }

    Ok(())
}

/*
===========================================
 Test Plan – Organize Command
===========================================

A. Dry-run Mode
----------------
- Preview only, no files moved.
- Given: 2 file categories, dry_run = true
- Assert:
  * Destination folders are NOT created.
  * No files are moved.

B. Move Mode — Normal Behavior
-------------------------------
- Move all files according to their categories.
- Given: valid source files.
- Assert:
  * Category folders are created.
  * Files are successfully moved.

- Case: Destination folder already exists
  * The folder exists before the operation.
  * Assert: No error should occur and processing continues.

C. Error Scenarios
-------------------
1. File not found
   * Assert: function still returns Ok(()).
   * Error message printed to stdout.

2. No permission to create folder
   * Use a read-only parent directory (OS dependent).
   * Expect: Err(FileError::PermissionDenied).

3. Rename/move fails due to permission
   * File is read-only.
   * Expect: Err(FileError::PermissionDenied).

D. Dynamic Category Behavior
-----------------------------
- Categories may change dynamically (e.g., "audio", "gif", etc).
- HashMap contains dynamic keys.
- Assert:
  * Function moves files based on the dynamic keys.
  * No hardcoded category assumptions.

E. Integration Behavior
------------------------
- Preview MUST always appear before the move operation.
- Use stdout capture.
- Expected output order:
  1) preview
  2) move operations
*/

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::fs::File;
    use tempfile::{Builder, TempDir};
    use crate::mover::{preview_file_moves};

    #[test]
    fn test_dry_run_mode() {
        let scan_dir = create_temp_dir("test_scan");

        let dest_dir = create_temp_dir("destination");

        let destination_path = dest_dir.path().to_str().unwrap();

        let exts = ["jpg", "mp4"];

        for ext in exts {
            let file_path = scan_dir.path().join(format!("file.{}", ext));
            File::create(file_path).unwrap();
        }

        let mut files = HashMap::new();
        files.insert(
            "images".to_string(),
            vec!["file.jpg".to_string()],
        );
        files.insert(
            "videos".to_string(),
            vec!["file.mp4".to_string()],
        );

        let result =  preview_file_moves(&files, scan_dir.path().to_str().unwrap(), destination_path, true);

        // Assert: destination folder should NOT contain category folders
        assert!(!dest_dir.path().join("images").exists());
        assert!(!dest_dir.path().join("videos").exists());
    }

    #[test]
    fn test_move_file() {
        let scan_dir = create_temp_dir("test_scan");

        let dest_dir = create_temp_dir("destination");

        let destination_path = dest_dir.path().to_str().unwrap();

        let exts = ["jpg", "mp4"];

        for ext in exts {
            let file_path = scan_dir.path().join(format!("file.{}", ext));
            File::create(file_path).unwrap();
        }

        let mut files = HashMap::new();
        files.insert(
            "images".to_string(),
            vec!["file.jpg".to_string()],
        );
        files.insert(
            "videos".to_string(),
            vec!["file.mp4".to_string()],
        );

        let result =  preview_file_moves(&files, scan_dir.path().to_str().unwrap(), destination_path, false);


    }


    fn create_temp_dir(dir_name : &str) -> TempDir {
        Builder::new()
            .prefix(dir_name)
            .tempdir()
            .expect("Failed to create temp dir")
    }
}

