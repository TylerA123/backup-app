use std::path::PathBuf;
use crate::db::LocalDb;

pub struct FileCache {
    db: std::sync::Arc<LocalDb>,
    cache_dir: PathBuf,
}

impl FileCache {
    pub fn new(db: std::sync::Arc<LocalDb>, cache_dir: PathBuf) -> Self {
        std::fs::create_dir_all(&cache_dir).ok();
        FileCache { db, cache_dir }
    }

    pub fn cache_path(&self, content_hash: &str) -> PathBuf {
        self.cache_dir.join(&content_hash[..2]).join(&content_hash[2..4]).join(content_hash)
    }

    pub fn has(&self, content_hash: &str) -> bool {
        self.cache_path(content_hash).exists()
    }
}
