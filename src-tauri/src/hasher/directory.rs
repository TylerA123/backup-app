use std::collections::HashMap;
use std::path::Path;
use crate::watcher::ExclusionRules;
use super::sha256::hash_file;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FileEntry {
    pub relative_path: String,
    pub content_hash: String,
    pub file_size: u64,
    pub last_modified: String,
}

pub type DirectorySnapshot = HashMap<String, FileEntry>;

pub fn hash_directory(
    root: &Path,
    exclusions: &ExclusionRules,
) -> Result<DirectorySnapshot, String> {
    let mut snapshot = DirectorySnapshot::new();
    walk_dir(root, root, exclusions, &mut snapshot)?;
    Ok(snapshot)
}

fn walk_dir(
    root: &Path,
    current: &Path,
    exclusions: &ExclusionRules,
    snapshot: &mut DirectorySnapshot,
) -> Result<(), String> {
    let dir = std::fs::read_dir(current)
        .map_err(|e| format!("Cannot read directory {}: {}", current.display(), e))?;

    for entry in dir {
        let entry = entry.map_err(|e| format!("Entry error: {}", e))?;
        let path = entry.path();
        let path_str = path.to_string_lossy().to_string();

        if exclusions.is_excluded(&path_str) {
            continue;
        }

        let relative = path
            .strip_prefix(root)
            .unwrap_or(&path)
            .to_string_lossy()
            .to_string()
            .replace('\\', "/");

        if path.is_dir() {
            walk_dir(root, &path, exclusions, snapshot)?;
        } else if path.is_file() {
            let metadata = std::fs::metadata(&path)
                .map_err(|e| format!("Metadata error {}: {}", path.display(), e))?;
            let file_size = metadata.len();

            // Quick skip for files too large to hash on the fly
            // They still get included, but hash is computed during upload
            let content_hash = if file_size > 500_000_000 {
                // Files > 500MB get placeholder hash, recomputed on upload
                format!("pending:{}", relative)
            } else {
                hash_file(&path)?
            };

            let last_modified = metadata
                .modified()
                .ok()
                .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
                .map(|d| d.as_secs().to_string())
                .unwrap_or_default();

            snapshot.insert(
                relative.clone(),
                FileEntry {
                    relative_path: relative,
                    content_hash,
                    file_size,
                    last_modified,
                },
            );
        }
    }
    Ok(())
}
