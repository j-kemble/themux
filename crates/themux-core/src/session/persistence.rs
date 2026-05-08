// Session persistence: auto-save timer, write/read snapshots.

use super::snapshot::AppSessionSnapshot;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{self, Duration};

/// Interval between auto-saves (8 seconds, matching macOS cmux).
pub const AUTOSAVE_INTERVAL_SECS: u64 = 8;

/// Manages session save and restore.
pub struct SessionPersistence {
    save_path: PathBuf,
    current_snapshot: Arc<Mutex<Option<AppSessionSnapshot>>>,
}

impl SessionPersistence {
    pub fn new() -> Self {
        let data_dir = dirs::data_dir()
            .unwrap_or_else(|| PathBuf::from("/tmp"))
            .join("themux");
        std::fs::create_dir_all(&data_dir).ok();
        let save_path = data_dir.join("session.json");
        let _previous_path = data_dir.join("session-previous.json");

        Self {
            save_path,
            current_snapshot: Arc::new(Mutex::new(None)),
        }
    }

    /// Start the auto-save background task.
    pub fn start_autosave(&self) -> tokio::task::JoinHandle<()> {
        let snapshot = self.current_snapshot.clone();
        let save_path = self.save_path.clone();

        tokio::spawn(async move {
            let mut interval = time::interval(Duration::from_secs(AUTOSAVE_INTERVAL_SECS));
            loop {
                interval.tick().await;
                let guard = snapshot.lock().await;
                if let Some(ref snap) = *guard {
                    if let Ok(json) = serde_json::to_string_pretty(snap) {
                        // Atomic write via temp file
                        let tmp_path = save_path.with_extension("tmp");
                        if std::fs::write(&tmp_path, &json).is_ok() {
                            std::fs::rename(&tmp_path, &save_path).ok();
                        }
                    }
                }
            }
        })
    }

    /// Update the in-memory snapshot.
    pub async fn update_snapshot(&self, snapshot: AppSessionSnapshot) {
        let mut guard = self.current_snapshot.lock().await;
        *guard = Some(snapshot);
    }

    /// Load the most recent snapshot from disk.
    pub fn load_snapshot(&self) -> Option<AppSessionSnapshot> {
        let content = std::fs::read_to_string(&self.save_path).ok()?;
        serde_json::from_str(&content).ok()
    }

    /// Save the "previous" snapshot (taken at quit time) for manual restore.
    pub fn save_previous_snapshot(&self) -> std::io::Result<()> {
        let previous_path = self.save_path.with_file_name("session-previous.json");
        if self.save_path.exists() {
            std::fs::copy(&self.save_path, &previous_path)?;
        }
        Ok(())
    }
}
