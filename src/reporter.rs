use std::collections::HashMap;
use crate::hasher;

pub fn find_duplicates(files: Vec<std::path::PathBuf>) -> HashMap<String, Vec<std::path::PathBuf>> {
    let mut duplicates: HashMap<String, Vec<std::path::PathBuf>> = HashMap::new();
    for file in files {
        let hash = match hasher::hash_file(&file) {
            Ok(hash) => hash,
            Err(_) => continue,
        };
        duplicates.entry(hash).or_insert(Vec::new()).push(file);
    }

    duplicates.retain(|_, files| files.len() > 1);
    duplicates
}

pub fn report_duplicates(duplicates: HashMap<String, Vec<std::path::PathBuf>>) {
    for (hash, files) in duplicates {
        println!("Hash: {}", hash);
        for file in files {
            println!(" - {}", file.display());
        }
    }
}
