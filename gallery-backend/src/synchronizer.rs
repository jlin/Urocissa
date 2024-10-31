use crate::executor::compressor::video_compressor::generate_compressed;
use crate::executor::executor;
use crate::public::error_data::{handle_error, ErrorData};
use crate::public::redb::DATA_TABLE;
use crate::public::tree::start_loop::SHOULD_RESET;
use crate::public::tree::TREE;

use arrayvec::ArrayString;
use std::panic::Location;
use std::sync::atomic::Ordering;
use std::sync::OnceLock;
use std::{
    collections::HashSet,
    path::PathBuf,
    sync::{Arc, Mutex},
};
use tokio;
use tokio::sync::mpsc::{unbounded_channel, UnboundedSender};

pub static EVENTS_SENDER: OnceLock<UnboundedSender<Vec<PathBuf>>> = OnceLock::new();
pub static VIDEO_QUEUE_SENDER: OnceLock<UnboundedSender<Vec<ArrayString<64>>>> = OnceLock::new();

pub async fn start_sync() -> anyhow::Result<()> {
    let (events_sender, mut events_receiver) = unbounded_channel::<Vec<PathBuf>>();
    EVENTS_SENDER.set(events_sender).unwrap();

    let (video_queue_sender, mut video_queue_receiver) =
        unbounded_channel::<Vec<ArrayString<64>>>();
    VIDEO_QUEUE_SENDER.set(video_queue_sender).unwrap();

    let events_repository: Arc<Mutex<Vec<PathBuf>>> = Arc::new(Mutex::new(Vec::new())); // Vector to store events
    let events_repository_clone: Arc<Mutex<Vec<PathBuf>>> = Arc::clone(&events_repository);

    let video_queue_repository: Arc<Mutex<Vec<ArrayString<64>>>> = Arc::new(Mutex::new(Vec::new())); // Vector to store video hash
    let video_queue_repository_repository_clone: Arc<Mutex<Vec<ArrayString<64>>>> =
        Arc::clone(&video_queue_repository);

    // Create a new thread to receive and process events
    tokio::task::spawn(async move {
        while let Some(event_paths) = events_receiver.recv().await {
            events_repository_clone
                .lock()
                .expect("events_repository_arc_clone lock error")
                .extend(event_paths);
        }
    });

    tokio::task::spawn(async move {
        while let Some(video_hash) = video_queue_receiver.recv().await {
            video_queue_repository_repository_clone
                .lock()
                .expect("events_repository_arc_clone lock error")
                .extend(video_hash);
        }
    });

    // CPU-intensive work is wrapped in tokio::task::spawn_blocking for async runtime
    tokio::task::spawn_blocking(move || {
        loop {
            std::thread::sleep(std::time::Duration::from_secs(3));
            let list_of_sync_files = {
                let mut events_repository_lock = events_repository
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
        }
    });

    // This thread is used to perform video_compression
    tokio::task::spawn_blocking(move || loop {
        std::thread::sleep(std::time::Duration::from_secs(3));
        let list_of_video_hash = {
            let mut video_queue_repository_repository_lock = video_queue_repository
                .lock()
                .expect("events_repository lock error");
            std::mem::take(&mut *video_queue_repository_repository_lock)
        };

        // Deduplication operation
        let hash_vec = list_of_video_hash
            .into_iter()
            .collect::<HashSet<ArrayString<64>>>()
            .into_iter()
            .collect::<Vec<ArrayString<64>>>();

        if !hash_vec.is_empty() {
            let read_table = TREE.read_tree_api();
            let database_vec = hash_vec.into_iter().filter_map(|hash| {
                match read_table.get(&*hash).unwrap() {
                    Some(guard) => {
                        // If this file is already in database
                        let database = guard.value();
                        Some(database)
                    }
                    None => None,
                }
            });
            database_vec.for_each(|mut database| match generate_compressed(&mut database) {
                Ok(_) => {
                    database.pending = false;
                    let write_txn = TREE.in_disk.begin_write().unwrap();
                    {
                        let mut write_table = write_txn.open_table(DATA_TABLE).unwrap();
                        write_table.insert(&*database.hash, &database).unwrap();
                    }
                    write_txn.commit().unwrap();
                    SHOULD_RESET.store(true, Ordering::SeqCst);
                }
                Err(error) => {
                    handle_error(ErrorData::new(
                        error.to_string(),
                        format!("An error occurred while processing file",),
                        Some(database.hash),
                        Some(database.imported_path()),
                        Location::caller(),
                    ));
                }
            })
        }
    });

    Ok(())
}
