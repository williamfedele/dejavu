use walkdir::WalkDir;

pub fn scan_directories(directories: Vec<std::path::PathBuf>) -> Vec<std::path::PathBuf>{
    let mut files = Vec::new();
    for directory in directories {
        for entry in WalkDir::new(directory) {
            let entry = entry.unwrap();
            if entry.file_type().is_file() {
                files.push(entry.path().to_path_buf());
            }
        }
    }
    files
}
