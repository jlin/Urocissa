use crate::executor::executor;

use log::info;
use std::mem;
use std::sync::OnceLock;
use std::{collections::HashSet, path::PathBuf};
use tokio;
use tokio::sync::mpsc::{unbounded_channel, UnboundedSender};

pub const BATCH_SIZE: usize = 100;
pub static EVENTS_SENDER: OnceLock<UnboundedSender<Vec<PathBuf>>> = OnceLock::new();

pub fn start_event_channel() -> tokio::task::JoinHandle<()> {
    let (events_sender, mut events_receiver) = unbounded_channel::<Vec<PathBuf>>();
    EVENTS_SENDER.set(events_sender).unwrap();

    tokio::task::spawn(async move {
        let mut buffer = Vec::new();

        while events_receiver.recv_many(&mut buffer, BATCH_SIZE).await > 0 {
            let start_time = std::time::Instant::now();

            let list_of_sync_files = mem::take(&mut buffer);
            info!(duration = &*format!("{:?}", start_time.elapsed()); "received events");

            tokio::task::spawn_blocking(move || {
                // Deduplicate the paths
                let unique_paths: HashSet<PathBuf> =
                    list_of_sync_files.into_iter().flatten().collect();
                let paths: Vec<PathBuf> = unique_paths.into_iter().collect();
                executor(paths);
            })
            .await
            .unwrap();
        }
    })
}
