use crate::config::{load_config, Config};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;


pub fn scan(path: &str) -> HashMap<String, Vec<String>> {
    // using test_file for testing
    let config = load_config();
    let mut groups = HashMap::new();

    for file in read_file(path) {
        match extract_extension(&file) {
           Some(ext) => {
               if let Some(category) = classify(&ext, &config) {
                   group_file(&mut groups, &category, &file);
               }else {
                   eprintln!("Unknown extension: {}", ext);
               }
           }
            None => {
                eprintln!("Unknown file type: {:?}", file);
            }
        }
    }

    print_groups(&mut groups);
    groups
}

fn read_file(path: &str) -> Vec<PathBuf> {
    WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .map(|e| e.path().to_path_buf())
        .collect()
}

fn extract_extension(path: &Path) -> Option<String> {
    path.extension()
        .and_then(|e| e.to_str())
        .map(|s| s.to_lowercase())
}

fn classify(ext: &str, config: &Config) -> Option<String> {
    if config.folders.archives.contains(&ext.to_string()) {
        Some("archive".to_string())
    }else if config.folders.documents.contains(&ext.to_string()) {
        Some("document".to_string())
    }else if config.folders.images.contains(&ext.to_string()) {
        Some("image".to_string())
    }else if config.folders.videos.contains(&ext.to_string()) {
        Some("video".to_string())
    }else {
        None
    }
}

fn group_file(groups: &mut HashMap<String, Vec<String>>, category: &str, path: &Path) {
    groups.entry(category.to_string())
        .or_default()
        .push(path.to_string_lossy().to_string());
}

fn print_groups(groups: &HashMap<String, Vec<String>>) {
    println!("== File Groups ==");
    for (key, value) in groups {
        println!("{}: {:?}", key, value);
    }

}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use tempfile::Builder;

    #[test]
    fn test_scan_grouping_logic() {
        let dir = Builder::new()
            .prefix("test_scan")
            .tempdir()
            .expect("Failed to create temp dir");

        let exts = [
            "jpg", "jpeg", "png", "gif", "pdf",
            "docx", "txt", "zip", "rar", "7z",
            "mp4"
        ];


        for ext in exts {
            let file_path = dir.path().join(format!("file.{}", ext));
            File::create(file_path).unwrap();
        }


        let result = scan(dir.path().to_str().unwrap());

        assert_eq!(result.get("archive").map_or(0, |v| v.len()), 3);
        assert_eq!(result.get("document").map_or(0, |v| v.len()), 3);
        assert_eq!(result.get("image").map_or(0, |v| v.len()), 4);
        assert_eq!(result.get("video").map_or(0, |v| v.len()), 1);
    }

}
