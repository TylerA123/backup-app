use std::path::PathBuf;
use std::sync::Arc;
use tracing::{info, warn, error};
use crate::db::LocalDb;
use crate::hasher::directory::hash_directory;
use crate::watcher::ExclusionRules;
use crate::sync::snapshot::SnapshotManager;
use crate::sync::dedup::DedupEngine;
use crate::upload::queue::UploadQueue;

pub struct SyncEngine {
    db: Arc<LocalDb>,
    snapshot_manager: SnapshotManager,
    dedup_engine: DedupEngine,
    upload_queue: UploadQueue,
    exclusions: ExclusionRules,
}

impl SyncEngine {
    pub fn new(db: Arc<LocalDb>) -> Self {
        let snapshot_manager = SnapshotManager::new(db.clone());
        let dedup_engine = DedupEngine::new(db.clone());
        let upload_queue = UploadQueue::new(db.clone());

        SyncEngine {
            db,
            snapshot_manager,
            dedup_engine,
            upload_queue,
            exclusions: ExclusionRules::new(),
        }
    }

    pub fn trigger_snapshot(&self, project_id: &str, project_path: &str) -> Result<String, String> {
        info!("Triggering snapshot for project: {}", project_id);

        let path = PathBuf::from(project_path);
        let snapshot = hash_directory(&path, &self.exclusions)?;

        let diff = self.snapshot_manager.compute_diff(project_id, &snapshot)?;

        let changed_hashes: Vec<String> = diff
            .added
            .iter()
            .chain(diff.modified.iter())
            .map(|e| e.content_hash.clone())
            .collect();

        let new_blobs = self.dedup_engine.find_new_blobs(&changed_hashes)?;

        for entry in diff.added.iter().chain(diff.modified.iter()) {
            if new_blobs.contains(&entry.content_hash) {
                let full_path = path.join(&entry.relative_path);
                self.upload_queue
                    .enqueue(
                        project_id,
                        &entry.relative_path,
                        &entry.content_hash,
                        entry.file_size as i64,
                        &full_path,
                    )
                    .map_err(|e| format!("Queue enqueue error: {}", e))?;
            }
        }

        let snapshot_id = diff.snapshot_id.clone();

        info!(
            "Snapshot {} complete: {} files, {} new blobs to upload",
            snapshot_id,
            diff.file_count,
            new_blobs.len()
        );

        Ok(snapshot_id)
    }
}
