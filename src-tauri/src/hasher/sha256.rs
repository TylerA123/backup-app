use sha2::{Sha256, Digest};
use std::io::Read;
use std::path::Path;
use std::fs::File;
use tracing::info;

pub fn hash_file(path: &Path) -> Result<String, String> {
    let mut file = File::open(path).map_err(|e| format!("Cannot open {}: {}", path.display(), e))?;
    let mut hasher = Sha256::new();
    let mut buffer = [0u8; 65536];

    loop {
        let bytes_read = file
            .read(&mut buffer)
            .map_err(|e| format!("Read error {}: {}", path.display(), e))?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }

    Ok(format!("{:x}", hasher.finalize()))
}

pub fn hash_file_with_progress<F: Fn(u64)>(
    path: &Path,
    total_size: u64,
    progress: F,
) -> Result<String, String> {
    let mut file = File::open(path).map_err(|e| format!("Cannot open {}: {}", path.display(), e))?;
    let mut hasher = Sha256::new();
    let mut buffer = [0u8; 65536];
    let mut processed: u64 = 0;

    loop {
        let bytes_read = file
            .read(&mut buffer)
            .map_err(|e| format!("Read error {}: {}", path.display(), e))?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
        processed += bytes_read as u64;
        progress(processed);
    }

    Ok(format!("{:x}", hasher.finalize()))
}
