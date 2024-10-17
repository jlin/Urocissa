use crate::executor::executor;
use std::sync::atomic::{AtomicBool, Ordering};
use std::{
    collections::HashSet,
    path::PathBuf,
    sync::{Arc, Mutex},
};
use tokio;
use tokio::sync::mpsc::UnboundedReceiver;

pub async fn start_sync(
    mut rx: UnboundedReceiver<Vec<PathBuf>>,
    turn_sync_on: Arc<AtomicBool>,
) -> anyhow::Result<()> {
    let events_repository: Arc<Mutex<Vec<PathBuf>>> = Arc::new(Mutex::new(Vec::new())); // Vector to store events
    let events_repository_clone: Arc<Mutex<Vec<PathBuf>>> = Arc::clone(&events_repository);
    // Create a new thread to receive and process events
    tokio::task::spawn(async move {
        while let Some(event_paths) = rx.recv().await {
            events_repository_clone
                .lock()
                .expect("events_repository_arc_clone lock error")
                .extend(event_paths);
        }
    });
    // Operations in the main thread
    // CPU-intensive work is wrapped in tokio::task::spawn_blocking for async runtime
    tokio::task::spawn_blocking(move || {
        loop {
            std::thread::sleep(std::time::Duration::from_secs(3));
            let events_repository_clone: Arc<Mutex<Vec<PathBuf>>> = Arc::clone(&events_repository);
            let list_of_sync_files = {
                let mut events_repository_lock = events_repository_clone
                    .lock()
                    .expect("events_repository lock error");
                std::mem::take(&mut *events_repository_lock)
            };
            // Deduplication operation
            let path = list_of_sync_files
                .into_iter()
                .collect::<HashSet<PathBuf>>()
                .into_iter()
                .collect::<Vec<PathBuf>>();

            if !path.is_empty() {
                executor(path);
            }

            if !turn_sync_on.load(Ordering::SeqCst) {
                println!("Stop Sync");
                break;
            }
        }
    });

    Ok(())
}
