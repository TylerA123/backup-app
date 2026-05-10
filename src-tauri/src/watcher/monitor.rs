use notify::{Config, Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::Path;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use tracing::{info, warn, error};

pub struct FileWatcher {
    watcher: Option<RecommendedWatcher>,
    running: Arc<AtomicBool>,
    rx: Option<mpsc::Receiver<notify::Result<Event>>>,
}

impl FileWatcher {
    pub fn new() -> Self {
        FileWatcher {
            watcher: None,
            running: Arc::new(AtomicBool::new(false)),
            rx: None,
        }
    }

    pub fn start<P: AsRef<Path>>(
        &mut self,
        path: P,
        callback: Box<dyn Fn(FileChange) + Send + 'static>,
    ) -> Result<(), String> {
        let (tx, rx) = mpsc::channel::<notify::Result<Event>>();
        self.rx = Some(rx);

        let mut watcher = notify::recommended_watcher(move |res| {
            let _ = tx.send(res);
        })
        .map_err(|e| format!("Failed to create watcher: {}", e))?;

        watcher
            .watch(path.as_ref(), RecursiveMode::Recursive)
            .map_err(|e| format!("Failed to watch path: {}", e))?;

        self.watcher = Some(watcher);
        self.running.store(true, Ordering::SeqCst);

        let running = self.running.clone();
        let rx = self.rx.take().unwrap();

        std::thread::spawn(move || {
            while running.load(Ordering::SeqCst) {
                match rx.recv() {
                    Ok(Ok(event)) => {
                        let change = FileChange::from_event(&event);
                        callback(change);
                    }
                    Ok(Err(e)) => {
                        warn!("Watch error: {}", e);
                    }
                    Err(mpsc::RecvError) => {
                        info!("Watcher channel closed");
                        break;
                    }
                }
            }
        });

        info!("File watcher started for: {:?}", path.as_ref());
        Ok(())
    }

    pub fn stop(&mut self) {
        self.running.store(false, Ordering::SeqCst);
        self.watcher = None;
        info!("File watcher stopped");
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FileChange {
    pub path: String,
    pub kind: ChangeKind,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum ChangeKind {
    Created,
    Modified,
    Deleted,
    Other,
}

impl FileChange {
    fn from_event(event: &Event) -> Self {
        let path = event
            .paths
            .first()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_default();

        let kind = match event.kind {
            EventKind::Create(_) => ChangeKind::Created,
            EventKind::Modify(_) => ChangeKind::Modified,
            EventKind::Remove(_) => ChangeKind::Deleted,
            _ => ChangeKind::Other,
        };

        FileChange { path, kind }
    }
}
