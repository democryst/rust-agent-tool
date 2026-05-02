use notify::{Watcher, RecursiveMode, Event};
use std::path::Path;
use anyhow::Result;
use tokio::sync::mpsc;

pub struct FileWatcher {
    watcher: notify::RecommendedWatcher,
}

impl FileWatcher {
    pub fn new(path: &str, tx: mpsc::Sender<Event>) -> Result<Self> {
        let mut watcher = notify::recommended_watcher(move |res: notify::Result<Event>| {
            if let Ok(event) = res {
                let _ = tx.blocking_send(event);
            }
        })?;

        watcher.watch(Path::new(path), RecursiveMode::Recursive)?;

        Ok(Self { watcher })
    }
}
