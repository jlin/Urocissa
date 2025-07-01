use std::collections::HashSet;
use std::fs;
use std::sync::OnceLock;
use std::{path::PathBuf, thread, time::Duration};
use tokio;
use tokio::sync::mpsc::{UnboundedSender, unbounded_channel};

pub static DELETE_QUEUE_SENDER: OnceLock<UnboundedSender<Vec<PathBuf>>> = OnceLock::new();
const MAX_DELETE_ATTEMPTS: u64 = 5;

pub fn start_delete_channel() -> tokio::task::JoinHandle<()> {
    let (delete_queue_sender, mut delete_queue_receiver) = unbounded_channel::<Vec<PathBuf>>();
    DELETE_QUEUE_SENDER.set(delete_queue_sender).unwrap();

    tokio::task::spawn(async move {
        while let Some(list_of_delete_path) = delete_queue_receiver.recv().await {
            tokio::task::spawn_blocking(move || {
                let unique_paths: HashSet<_> = list_of_delete_path.into_iter().collect();
                let path_vec: Vec<_> = unique_paths.into_iter().collect();

                for path in path_vec {
                    let mut attempts = 0;
                    loop {
                        attempts += 1;
                        match fs::remove_file(&path) {
                            Ok(_) => {
                                log::info!("Deleted file: {:?}", path);
                                break;
                            }
                            Err(e) if attempts < MAX_DELETE_ATTEMPTS => {
                                log::warn!(
                                    "Failed deleting {:?} (attempt {}), retrying in {}ms: {}",
                                    path,
                                    attempts,
                                    100 * attempts,
                                    e
                                );
                                thread::sleep(Duration::from_millis(100 * attempts));
                                continue;
                            }
                            Err(e) => {
                                log::error!(
                                    "Failed deleting {:?} after {} attempts: {}",
                                    path,
                                    attempts,
                                    e
                                );
                                break;
                            }
                        }
                    }
                }
            })
            .await
            .unwrap();
        }
    })
}

pub fn delete_paths(paths: Vec<PathBuf>) {
    DELETE_QUEUE_SENDER
        .get()
        .expect("Delete channel not initialized")
        .send(paths)
        .unwrap();
}

