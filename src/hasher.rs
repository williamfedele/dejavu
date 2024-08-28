use sha2::{Sha256, Digest};
use std::fs::File;
use std::io::{BufReader, Read};

pub fn hash_file(file: &std::path::PathBuf) -> Result<String, std::io::Error> {
    let mut reader = BufReader::new(File::open(file)?);
    let mut hasher = Sha256::new();
    let mut buffer = [0; 1024];
    loop {
        let n = reader.read(&mut buffer)?;
        if n == 0 {
            break;
        }
        hasher.update(&buffer[..n]);
    }
    Ok(format!("{:x}", hasher.finalize()))
}
