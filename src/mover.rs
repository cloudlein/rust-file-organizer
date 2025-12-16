use colored::Colorize;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::{fs, io};
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

pub fn move_files(
    root_path: &str,
    destination: &str,
    files: &HashMap<String, Vec<String>>,
    dry_run: bool,
) -> Result<(), FileError> {
    preview_file_moves(files, root_path, destination, dry_run);

    if dry_run {
        return Ok(());
    }

    for (category, items) in files {
        let dest_dir = Path::new(destination).join(category);

        if let Err(e) = create_folder_and_move_files(
            category,
            &dest_dir,
            items,
            root_path,
        ) {
            eprintln!("{e}");
            return Err(e);
        }
    }

    Ok(())
}


fn preview_file_moves(
    files: &HashMap<String, Vec<String>>,
    path_scan: &str,
    destination: &str,
    dry_run: bool,
) {
    let mut file_moves = Vec::new();

    for (category, items) in files {
        for item in items {
            file_moves.push(FileMove {
                before: format!("{}/{}/{}", path_scan, category, item),
                after: format!("{}/{}/{}", destination, category, item),
            });
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
    category: &str,
    dest_dir: &Path,
    file_paths: &[String],
    src_root: &str,
) -> Result<(), FileError> {

    std::fs::create_dir_all(dest_dir).map_err(|err| {
        if err.kind() == std::io::ErrorKind::PermissionDenied {
            FileError::PermissionDenied { path: dest_dir.into() }
        } else {
            FileError::FolderCreation {
                path: dest_dir.into(),
                source: err,
            }
        }
    })?;

    let src_base = Path::new(src_root).join(category);

    for file in file_paths {
        let src = src_base.join(file);

        if !src.exists() {
            eprintln!("{}", FileError::NotFound { path: src.clone() });
            continue;
        }

        let dest = dest_dir.join(file);

        if dest.exists() {
            eprintln!("{}", FileError::DestinationExists { path: dest.clone() });
            continue;
        }

        std::fs::rename(&src, &dest).map_err(|e| match e.kind() {
            std::io::ErrorKind::PermissionDenied => FileError::PermissionDenied {
                path: src.clone(),
            },
            _ => FileError::FileMove {
                path: src.clone(),
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
    use crate::mover::{move_files, preview_file_moves, FileError};
    use std::collections::HashMap;
    use std::fs;
    use std::fs::File;
    use std::io::{ErrorKind, Read, Write};
    use tempfile::{Builder, TempDir};

    type MediaExtensions = HashMap<&'static str, Vec<&'static str>>;

    #[test]
    fn test_dry_run_mode() {
        let scan_dir = create_temp_dir("test_scan");
        let dest_dir = create_temp_dir("destination");
        let destination_path = dest_dir.path().to_str().unwrap();

        let files = create_temp_file_per_category(
            [
              ("images".to_string(), vec!["jpg".to_string(), "png".to_string()]),
              ("videos".to_string(), vec!["mp4".to_string()]),
              ("audio".to_string(), vec!["mp3".to_string(), "wav".to_string()]),
            ]
            .into_iter()
            .collect(),
            &scan_dir);

        let result = preview_file_moves(&files, scan_dir.path().to_str().unwrap(), destination_path, true);

        assert!(!dest_dir.path().join("images").exists());
        assert!(!dest_dir.path().join("videos").exists());
    }

    #[test]
    fn test_move_file_mode() {
        let scan_dir = create_temp_dir("scan");
        let dest_dir = create_temp_dir("dest");

        let files = create_temp_file_per_category(
            [
                ("images".to_string(), vec!["file.jpg".to_string(), "file.png".to_string()]),
                ("videos".to_string(), vec!["file.mp4".to_string()]),
                ("audio".to_string(), vec!["file.mp3".to_string(), "file.wav".to_string()]),
            ]
                .into_iter()
                .collect(),
            &scan_dir,
        );

        let result = move_files(
            scan_dir.path().to_str().unwrap(),
            dest_dir.path().to_str().unwrap(),
            &files,
            false,
        );

        assert!(result.is_ok());

        assert!(dest_dir.path().join("images/file.jpg").exists());
        assert!(dest_dir.path().join("images/file.png").exists());
        assert!(dest_dir.path().join("videos/file.mp4").exists());
        assert!(dest_dir.path().join("audio/file.mp3").exists());
    }

    #[test]
    #[ignore]
    fn test_move_mode_missing_file_returns_ok_and_logs_error() {
        let scan_dir = create_temp_dir("test_scan");
        let dest_dir = create_temp_dir("destination");
        let destination_path = dest_dir.path().to_str().unwrap();

        // Capture stderr
        let mut stderr_buf = gag::BufferRedirect::stderr().unwrap();

        let mut files = HashMap::new();
        files.insert("images".to_string(), vec!["file.jpg".to_string()]);
        files.insert("videos".to_string(), vec!["file.mp4".to_string()]);

        let result = move_files(
            scan_dir.path().to_str().unwrap(),
            destination_path,
            &files,
            false,
        );

        // flush agar semua output masuk buffer
        std::io::stderr().flush().unwrap();

        let mut captured_output = String::new();
        stderr_buf.read_to_string(&mut captured_output).unwrap();


        // sekarang assert akan valid
        assert!(captured_output.contains("file.jpg") && captured_output.contains("not found"));
        assert!(captured_output.contains("file.mp4") && captured_output.contains("not found"));

        assert!(!dest_dir.path().join("/images/file.jpg").exists());
        assert!(!dest_dir.path().join("/videos/file.mp4").exists());

        assert!(result.is_ok());
    }


    #[test]
    fn test_move_mode_permission_denied_on_create_folder() {
        use std::os::unix::fs::PermissionsExt;
        let scan_dir = create_temp_dir("test_scan");
        let dest_dir = create_temp_dir("destination");
        let destination_path = dest_dir.path().to_str().unwrap();

        // Lock folder completely → no read/write/execute
        std::fs::set_permissions(
            destination_path,
            std::fs::Permissions::from_mode(0o000)
        ).unwrap();

        let mut stderr_buf = gag::BufferRedirect::stderr().unwrap();

        let mut files = HashMap::new();
        files.insert("images".to_string(), vec!["file.jpg".to_string()]);
        files.insert("videos".to_string(), vec!["file.mp4".to_string()]);

        let result = move_files(
            scan_dir.path().to_str().unwrap(),
            destination_path,
            &files,
            false,
        );

        std::io::stderr().flush().unwrap();

        let mut captured_output = String::new();
        stderr_buf.read_to_string(&mut captured_output).unwrap();

        assert!(matches!(
            result,
            Err(FileError::PermissionDenied { .. })
        ));
    }

    #[test]
    fn test_move_mode_permission_denied_on_move_file() {
        use std::os::unix::fs::PermissionsExt;

        let scan_dir = create_temp_dir("test_scan");
        let dest_dir = create_temp_dir("destination");
        let destination_path = dest_dir.path().to_str().unwrap();

        let mut files = HashMap::new();
        files.insert("images".to_string(), vec!["file.jpg".to_string()]);
        files.insert("videos".to_string(), vec!["file.mp4".to_string()]);

        for (_, list_file) in &files {
            for file in list_file {
                File::create(scan_dir.path().join(file)).unwrap();
            }
        }

        let mut perm = dest_dir.path().metadata().unwrap().permissions();
        perm.set_mode(0o555);
        fs::set_permissions(dest_dir.path(), perm).unwrap();

        let result = move_files(
            scan_dir.path().to_str().unwrap(),
            destination_path,
            &files,
            false,
        );

        assert!(matches!(
        result,
        Err(FileError::PermissionDenied { .. })
    ));
    }

    #[test]
     fn test_move_mode_moves_files_based_on_dynamic_category_map() {
         let scan_dir = create_temp_dir("scan");
         let dest_dir = create_temp_dir("dest");

         let files = create_temp_file_per_category(
             [
                 ("weird_cat".to_string(), vec!["a.bin".to_string()]),
                 ("☃️snow".to_string(), vec!["b.raw".to_string()]),
                 ("2025".to_string(), vec!["c.dat".to_string()]),
             ]
                 .into_iter()
                 .collect(),
             &scan_dir
         );

         let result = move_files(
             scan_dir.path().to_str().unwrap(),
             dest_dir.path().to_str().unwrap(),
             &files,
             false,
         );

         assert!(result.is_ok());

         assert!(dest_dir.path().join("weird_cat/a.bin").exists());
         assert!(dest_dir.path().join("☃️snow/b.raw").exists());
         assert!(dest_dir.path().join("2025/c.dat").exists());

     }


    // test_integration_preview_then_move_output_order()
    // test_preview_is_printed_before_move_operations()

    fn create_temp_dir(dir_name : &str) -> TempDir {
        Builder::new()
            .prefix(dir_name)
            .tempdir()
            .expect("Failed to create temp dir")
    }

    fn create_temp_file_per_category(
        files: HashMap<String, Vec<String>>,
        scan_dir: &TempDir,
    ) -> HashMap<String, Vec<String>> {
        for (category, file_names) in &files {
            let category_dir = scan_dir.path().join(category);
            std::fs::create_dir_all(&category_dir).unwrap();

            for file_name in file_names {
                let file_path = category_dir.join(file_name);
                File::create(&file_path).unwrap();
            }
        }

        files
    }

}

