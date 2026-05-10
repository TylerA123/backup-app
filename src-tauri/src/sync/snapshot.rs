use std::collections::HashSet;
use serde::{Serialize, Deserialize};
use sha2::Digest;
use crate::hasher::directory::{DirectorySnapshot, FileEntry};
use crate::db::LocalDb;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotDiff {
    pub snapshot_id: String,
    pub project_id: String,
    pub added: Vec<FileEntry>,
    pub modified: Vec<FileEntry>,
    pub deleted: Vec<String>,
    pub unchanged: Vec<String>,
    pub fingerprint: String,
    pub total_size: u64,
    pub file_count: usize,
}

pub struct SnapshotManager {
    db: std::sync::Arc<LocalDb>,
}

impl SnapshotManager {
    pub fn new(db: std::sync::Arc<LocalDb>) -> Self {
        SnapshotManager { db }
    }

    pub fn compute_diff(
        &self,
        project_id: &str,
        current: &DirectorySnapshot,
    ) -> Result<SnapshotDiff, String> {
        let previous = self.load_latest_snapshot(project_id)?;
        let diff = self.diff_snapshots(project_id, &previous, current)?;
        Ok(diff)
    }

    fn load_latest_snapshot(&self, project_id: &str) -> Result<DirectorySnapshot, String> {
        // In MVP, we compare against what's currently on disk by re-hashing
        // Future: store previous snapshot manifest in SQLite
        Ok(DirectorySnapshot::new())
    }

    fn diff_snapshots(
        &self,
        project_id: &str,
        previous: &DirectorySnapshot,
        current: &DirectorySnapshot,
    ) -> Result<SnapshotDiff, String> {
        let mut added = Vec::new();
        let mut modified = Vec::new();
        let mut deleted = Vec::new();
        let mut unchanged = Vec::new();
        let mut total_size: u64 = 0;

        let prev_paths: HashSet<&String> = previous.keys().collect();
        let curr_paths: HashSet<&String> = current.keys().collect();

        for path in &curr_paths {
            if let Some(entry) = current.get(*path) {
                total_size += entry.file_size;
                if let Some(prev_entry) = previous.get(*path) {
                    if prev_entry.content_hash == entry.content_hash {
                        unchanged.push((*path).clone());
                    } else {
                        modified.push(entry.clone());
                    }
                } else {
                    added.push(entry.clone());
                }
            }
        }

        for path in prev_paths.difference(&curr_paths) {
            deleted.push((*path).clone());
        }

        let snapshot_id = uuid::Uuid::new_v4().to_string();
        let fingerprint = self.compute_fingerprint(current)?;

        Ok(SnapshotDiff {
            snapshot_id,
            project_id: project_id.to_string(),
            added,
            modified,
            deleted,
            unchanged,
            fingerprint,
            total_size,
            file_count: current.len(),
        })
    }

    fn compute_fingerprint(&self, snapshot: &DirectorySnapshot) -> Result<String, String> {
        let mut entries: Vec<&FileEntry> = snapshot.values().collect();
        entries.sort_by(|a, b| a.relative_path.cmp(&b.relative_path));

        let canonical = entries
            .iter()
            .map(|e| format!("{}:{}:{}", e.relative_path, e.content_hash, e.file_size))
            .collect::<Vec<_>>()
            .join("\n");

        Ok(sha2::Sha256::digest(canonical.as_bytes()).to_vec()
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect::<String>())
    }
}
