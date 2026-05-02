use notify::{Watcher, RecursiveMode, Event};
use std::path::Path;
use anyhow::Result;
use tokio::sync::mpsc;

pub struct FileWatcher {
    _watcher: notify::RecommendedWatcher,
}

impl FileWatcher {
    pub fn new(path: impl AsRef<Path>, tx: mpsc::Sender<Event>) -> Result<Self> {
        let mut watcher = notify::recommended_watcher(move |res: Result<Event, notify::Error>| {
            if let Ok(event) = res {
                let _ = tx.blocking_send(event);
            }
        })?;

        watcher.watch(path.as_ref(), RecursiveMode::Recursive)?;

        Ok(Self { _watcher: watcher })
    }
}
