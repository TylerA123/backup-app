use std::collections::HashSet;
use std::time::{Duration, Instant};

pub struct Debouncer {
    debounce_ms: u64,
    pending_changes: HashSet<String>,
    last_event: Option<Instant>,
}

impl Debouncer {
    pub fn new(debounce_ms: u64) -> Self {
        Debouncer {
            debounce_ms,
            pending_changes: HashSet::new(),
            last_event: None,
        }
    }

    pub fn record_change(&mut self, path: String) {
        self.pending_changes.insert(path);
        self.last_event = Some(Instant::now());
    }

    pub fn is_ready(&self) -> bool {
        match self.last_event {
            Some(time) => time.elapsed() >= Duration::from_millis(self.debounce_ms),
            None => false,
        }
    }

    pub fn drain_changes(&mut self) -> Vec<String> {
        let changes: Vec<String> = self.pending_changes.drain().collect();
        self.last_event = None;
        changes
    }

    pub fn reset(&mut self) {
        self.pending_changes.clear();
        self.last_event = None;
    }
}
