use std::collections::HashMap;
use crate::db::LocalDb;

pub struct DedupEngine {
    db: std::sync::Arc<LocalDb>,
}

impl DedupEngine {
    pub fn new(db: std::sync::Arc<LocalDb>) -> Self {
        DedupEngine { db }
    }

    pub fn find_new_blobs(
        &self,
        hashes: &[String],
    ) -> Result<Vec<String>, String> {
        let mut existing = self.get_known_hashes()?;
        let new: Vec<String> = hashes
            .iter()
            .filter(|h| !existing.contains_key(*h))
            .cloned()
            .collect();
        Ok(new)
    }

    fn get_known_hashes(&self) -> Result<HashMap<String, bool>, String> {
        // In MVP, check local SQLite for cached hashes
        // In production, also check Supabase blob_references
        let pending = self.db.get_pending_uploads().map_err(|e| e.to_string())?;
        let mut known = HashMap::new();
        for item in pending {
            known.insert(item.content_hash, true);
        }
        Ok(known)
    }
}
