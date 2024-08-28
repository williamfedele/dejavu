use std::error::Error;
use std::path::PathBuf;
use walkdir::WalkDir;

pub fn scan_directories(
    directories: Vec<std::path::PathBuf>,
) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    let mut files = Vec::new();
    for directory in directories {
        for entry in WalkDir::new(directory) {
            let entry = entry.unwrap(); // TODO
            if entry.file_type().is_file() {
                files.push(entry.path().to_path_buf());
            }
        }
    }
    Ok(files)
}
