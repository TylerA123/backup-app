use std::sync::Arc;
use tracing::{info, warn};

pub struct UploadWorker {
    max_concurrent: usize,
}

impl UploadWorker {
    pub fn new(max_concurrent: usize) -> Self {
        UploadWorker { max_concurrent }
    }

    pub fn max_concurrent(&self) -> usize {
        self.max_concurrent
    }
}
