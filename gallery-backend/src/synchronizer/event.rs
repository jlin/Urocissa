use crate::executor::executor;

use log::info;
use std::sync::OnceLock;
use std::{collections::HashSet, path::PathBuf};
use tokio;
use tokio::sync::mpsc::{self, unbounded_channel, UnboundedSender};

pub const BATCH_SIZE: usize = 100;
pub static EVENTS_SENDER: OnceLock<UnboundedSender<Vec<PathBuf>>> = OnceLock::new();

pub fn start_event_channel() -> tokio::task::JoinHandle<()> {
    let (events_sender, mut events_receiver) = unbounded_channel::<Vec<PathBuf>>();
    EVENTS_SENDER.set(events_sender).unwrap();

    tokio::task::spawn(async move {
        while let Some(list_of_sync_files) = events_receiver.recv().await {
            // Initialize a batch with the received list
            let mut batch = Vec::with_capacity(BATCH_SIZE);
            batch.extend(list_of_sync_files);

            // Attempt to drain additional items without waiting
            let start_time = std::time::Instant::now();
            while batch.len() < BATCH_SIZE {
                match events_receiver.try_recv() {
                    Ok(mut more_files) => {
                        batch.append(&mut more_files);
                    }
                    Err(mpsc::error::TryRecvError::Empty) => {
                        // No more items are immediately available
                        break;
                    }
                    Err(mpsc::error::TryRecvError::Disconnected) => {
                        // Sender has been dropped; stop collecting
                        break;
                    }
                }
            }
            info!(duration = &*format!("{:?}", start_time.elapsed()); "received events");

            // Deduplicate the paths
            let unique_paths: HashSet<PathBuf> = batch.into_iter().collect();
            let paths: Vec<PathBuf> = unique_paths.into_iter().collect();

            // Spawn a blocking task to handle the batch
            let paths_clone = paths.clone();
            tokio::task::spawn_blocking(move || {
                executor(paths_clone);
            })
            .await
            .unwrap();
        }
    })
}
