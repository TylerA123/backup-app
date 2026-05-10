use std::path::PathBuf;
use std::sync::Arc;
use tracing::info;
use crate::db::LocalDb;

pub struct UploadQueue {
    db: Arc<LocalDb>,
}

impl UploadQueue {
    pub fn new(db: Arc<LocalDb>) -> Self {
        UploadQueue { db }
    }

    pub fn enqueue(
        &self,
        project_id: &str,
        relative_path: &str,
        content_hash: &str,
        file_size: i64,
        _full_path: &PathBuf,
    ) -> Result<(), String> {
        self.db
            .add_to_upload_queue(project_id, relative_path, content_hash, file_size)
            .map_err(|e| e.to_string())?;
        info!("Enqueued {} ({}) for upload", relative_path, bytesize::ByteSize::b(file_size as u64));
        Ok(())
    }

    pub fn pending_count(&self) -> Result<i64, String> {
        let items = self.db.get_pending_uploads().map_err(|e| e.to_string())?;
        Ok(items.len() as i64)
    }
}
