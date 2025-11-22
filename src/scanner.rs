use walkdir::WalkDir;
use crate::config::{load_config, Config};
pub struct FileGroups {
    pub archives: Vec<String>,
    pub documents: Vec<String>,
    pub images: Vec<String>,
    pub videos: Vec<String>,
}

impl FileGroups {
    pub fn new() -> Self {
        Self {
            archives: vec![],
            documents: vec![],
            images: vec![],
            videos: vec![],
        }
    }
}

enum FileType {
    Archive,
    Image,
    Document,
    Unsupported,
    Video
}

pub fn scan(path: &str) -> FileGroups{
    // using test_file for testing
    let config = load_config();
    let mut groups = FileGroups::new();

    for entry in WalkDir::new(path) {
        let entry = match entry {
            Ok(e) => e,
            Err(e) => {
                eprintln!("Failed to read entry: {}", e);
                continue;
            }
        };

        let path = entry.path();

        if !path.is_file() {
            continue;
        }

        let ext = match path.extension().and_then(|e| e.to_str()) {
            Some(e) => e.to_lowercase(),
            None => {
                eprintln!("No extension: {:?}", path);
                continue;
            }
        };

        match detect_file_type(&ext, &config) {
            FileType::Archive => groups.archives.push(path.to_string_lossy().to_string()),
            FileType::Image => groups.images.push(path.to_string_lossy().to_string()),
            FileType::Document => groups.documents.push(path.to_string_lossy().to_string()),
            FileType::Video => groups.videos.push(path.to_string_lossy().to_string()),
            FileType::Unsupported => {
                eprintln!("⚠️ Unsupported file type: {}", ext);
            }
        }
    }

    print_groups(&mut groups);
    groups
}

fn detect_file_type(ext: &str, config: &Config) -> FileType {
    let ext = ext.to_lowercase();

    if config.folders.archives.contains(&ext) {
        FileType::Archive
    } else if config.folders.images.contains(&ext) {
        FileType::Image
    } else if config.folders.documents.contains(&ext) {
        FileType::Document
    } else if config.folders.videos.contains(&ext) {
        FileType::Video
    } else {
        FileType::Unsupported
    }
}

fn print_groups(groups: &FileGroups) {
    println!("== File Groups ==");

    println!("Archives:");
    for item in &groups.archives {
        println!("  - {}", item);
    }

    println!("Documents:");
    for item in &groups.documents {
        println!("  - {}", item);
    }

    println!("Images:");
    for item in &groups.images {
        println!("  - {}", item);
    }

    println!("Videos:");
    for item in &groups.videos {
        println!("  - {}", item);
    }
}


#[cfg(test)]
mod tests {
    use std::fs::File;
    use super::*;
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

        assert_eq!(result.archives.len(), 3);
        assert_eq!(result.documents.len(), 3);
        assert_eq!(result.images.len(), 4);
        assert_eq!(result.videos.len(), 1);

    }

}
