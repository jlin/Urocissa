use crate::constant::PROCESS_BATCH_NUMBER;
use crate::coordinator::index::IndexTask;
use crate::coordinator::{COORDINATOR, Task};

use log::info;
use std::sync::{Arc, OnceLock};
use std::{collections::HashSet, path::PathBuf};
use tokio;
use tokio::sync::Notify;
use tokio::sync::mpsc::{UnboundedSender, unbounded_channel};

#[derive(Debug)]
pub struct ScanQueue {
    pub scan_list: Vec<PathBuf>,
    pub notify: Option<Arc<Notify>>,
}

pub static EVENTS_SENDER: OnceLock<UnboundedSender<ScanQueue>> = OnceLock::new();

pub fn start_event_channel() -> tokio::task::JoinHandle<()> {
    let (events_sender, mut events_receiver) = unbounded_channel::<ScanQueue>();
    EVENTS_SENDER.set(events_sender).unwrap();

    tokio::task::spawn(async move {
        loop {
            let mut buffer = Vec::new();
            events_receiver
                .recv_many(&mut buffer, PROCESS_BATCH_NUMBER)
                .await;
            tokio::task::spawn_blocking(move || {
                let start_time = std::time::Instant::now();

                info!(duration = &*format!("{:?}", start_time.elapsed()); "received events");

                // Collect unique paths and notification objects in a single pass
                let mut unique_paths = HashSet::new();
                let mut notify_list = Vec::new();

                for queue in buffer {
                    // Add paths directly to the set
                    unique_paths.extend(queue.scan_list);
                    // Collect notifications if present
                    if let Some(notify) = queue.notify {
                        notify_list.push(notify);
                    }
                }

                // Convert to Vec only once
                let paths: Vec<PathBuf> = unique_paths.into_iter().collect();
                for path in paths {
                    if let Err(err) = COORDINATOR.submit(Task::Index(IndexTask::new(path))) {
                        error!("Failed to submit task:\n{:#}", err);
                    }
                }
                // Notify all at once
                for notify in notify_list {
                    notify.notify_one();
                }
            })
            .await
            .unwrap();
        }
    })
}
