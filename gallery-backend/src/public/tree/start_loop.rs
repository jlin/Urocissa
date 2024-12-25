use super::Tree;
use crate::public::abstract_data::AbstractData;
use crate::public::database_struct::database_timestamp::DataBaseTimestamp;
use crate::public::expire::start_loop::{NEXT_EXPIRE_TIME, SHOULD_CHECK_QUERY_EXPIRE};
use crate::public::expire::{EXPIRE, EXPIRE_TABLE_DEFINITION};
use crate::public::redb::{ALBUM_TABLE, DATA_TABLE};
use crate::public::utils::{get_current_timestamp_u64, info_wrap};
use crate::router::put::edit_album::AlbumQueue;
use crate::synchronizer::album::ALBUM_QUEUE_SENDER;

use arrayvec::ArrayString;
use log::info;
use rayon::iter::{ParallelBridge, ParallelIterator};
use rayon::prelude::ParallelSliceMut;
use redb::ReadableTable;
use std::collections::HashSet;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{LazyLock, OnceLock};
use std::time::{Duration, Instant};
use std::usize;
use tokio::sync::mpsc::{unbounded_channel, UnboundedSender};
use tokio::sync::Notify;

static ALLOWED_KEYS: LazyLock<HashSet<&'static str>> = LazyLock::new(|| {
    [
        "Make",
        "Model",
        "FNumber",
        "ExposureTime",
        "FocalLength",
        "PhotographicSensitivity",
        "DateTimeOriginal",
        "duration",
        "rotation",
    ]
    .iter()
    .cloned()
    .collect()
});

pub static ALBUM_WAITING_FOR_MEMORY_UPDATE_SENDER: OnceLock<UnboundedSender<AlbumQueue>> =
    OnceLock::new();

pub static SHOULD_RESET: Notify = Notify::const_new();

pub static VERSION_COUNT_TIMESTAMP: AtomicU64 = AtomicU64::new(0);
impl Tree {
    /// Starts an asynchronous loop that listens for reset notifications and updates the in-memory cache.
    ///
    /// This function spawns a Tokio task that continuously waits for a `SHOULD_RESET` notification.
    /// Upon receiving the notification, it collects any pending album updates and spawns a blocking task
    /// to update the in-memory cache from on-disk data. It also manages expiration times and sends
    /// album updates to the appropriate queue.
    ///
    /// # Returns
    ///
    /// A `JoinHandle` for the spawned Tokio task.
    pub fn start_loop(&self) -> tokio::task::JoinHandle<()> {
        // Spawn an asynchronous Tokio task
        tokio::task::spawn(async {
            // Create an unbounded channel for receiving album updates
            let (album_waiting_for_update_sender, mut album_waiting_for_update_receiver) =
                unbounded_channel::<AlbumQueue>();

            // Initialize the global sender with the sender end of the channel
            ALBUM_WAITING_FOR_MEMORY_UPDATE_SENDER
                .set(album_waiting_for_update_sender)
                .unwrap();

            // Enter an infinite loop to continuously listen for reset notifications
            loop {
                // Await a notification indicating that a reset should occur
                SHOULD_RESET.notified().await;

                // Initialize a buffer to collect album updates
                let mut buffer = Vec::new();

                // If there are pending album updates, receive them into the buffer
                if !album_waiting_for_update_receiver.is_empty() {
                    // Receive as many album updates as possible (up to usize::MAX)
                    album_waiting_for_update_receiver
                        .recv_many(&mut buffer, usize::MAX)
                        .await;
                }

                // Spawn a blocking task to handle the reset and update operations
                tokio::task::spawn_blocking(|| {
                    let start_time = Instant::now();
                    // Begin a read transaction on the in-disk data store
                    let table = self
                        .in_disk
                        .begin_read()
                        .unwrap()
                        .open_table(DATA_TABLE)
                        .unwrap();

                    // Define the priority list for sorting
                    let priority_list =
                        vec!["DateTimeOriginal", "filename", "modified", "scan_time"];

                    // Collect data from the DATA_TABLE into a vector of `DataBaseTimestamp`
                    let mut data_vec: Vec<DataBaseTimestamp> = table
                        .iter()
                        .unwrap()
                        .par_bridge()
                        .map(|guard| {
                            let (_key, value) = guard.unwrap();
                            let mut database = value.value();
                            database
                                .exif_vec
                                .retain(|k, _| ALLOWED_KEYS.contains(&k.as_str()));
                            DataBaseTimestamp::new(AbstractData::DataBase(database), &priority_list)
                        })
                        .collect();

                    // Open the ALBUM_TABLE for reading
                    let album_table = self
                        .in_disk
                        .begin_read()
                        .unwrap()
                        .open_table(ALBUM_TABLE)
                        .unwrap();

                    // Collect data from the ALBUM_TABLE into a vector of `DataBaseTimestamp`
                    let album_vec: Vec<DataBaseTimestamp> = album_table
                        .iter()
                        .unwrap()
                        .par_bridge()
                        .map(|guard| {
                            let (_key, value) = guard.unwrap();
                            let album = value.value();
                            DataBaseTimestamp::new(AbstractData::Album(album), &priority_list)
                        })
                        .collect();

                    // Extend the main data vector with album data
                    data_vec.extend(album_vec);

                    // Sort the combined data vector in parallel based on timestamps (descending order)
                    data_vec.par_sort_by(|a, b| b.timestamp.cmp(&a.timestamp));

                    // Acquire a write lock on the in-memory cache and update it with the sorted data
                    *self.in_memory.write().unwrap() = data_vec;

                    // Retrieve the current timestamp
                    let current_timestamp = get_current_timestamp_u64();

                    // Atomically swap the `VERSION_COUNT_TIMESTAMP` with the current timestamp and get the last timestamp
                    let last_timestamp =
                        VERSION_COUNT_TIMESTAMP.swap(current_timestamp, Ordering::SeqCst);

                    // Log that the in-memory cache has been updated
                    info_wrap(
                        Some(start_time.elapsed()),
                        &format!("In-memory cache updated ({}).", current_timestamp),
                    );

                    // Begin a write transaction on the EXPIRE table in the in-disk data store
                    let expire_write_txn = EXPIRE.in_disk.begin_write().unwrap();

                    // If there was a previous timestamp, update the expire table
                    if last_timestamp > 0 {
                        // Calculate the new expire time by adding 1 hour (in milliseconds) to the current timestamp
                        let new_expire_time = current_timestamp
                            .saturating_add(Duration::from_secs(60 * 60).as_millis() as u64);

                        {
                            // Open the EXPIRE_TABLE_DEFINITION for writing
                            let mut expire_table = expire_write_txn
                                .open_table(EXPIRE_TABLE_DEFINITION)
                                .expect("Failed to open expire table");

                            // Insert the last_timestamp with the new expire time into the expire table
                            expire_table
                                .insert(last_timestamp, Some(new_expire_time))
                                .expect("Failed to insert into expire table");

                            // Insert the current_timestamp with `None` to indicate that it has no expiration
                            expire_table
                                .insert(current_timestamp, None)
                                .expect("Failed to insert into expire table");

                            // Log that the expire table has been updated with the new expire time
                            info!(
                                "Expire table updated. Next expire time set to {}",
                                new_expire_time
                            );
                        }

                        // Commit the write transaction to finalize changes to the expire table
                        expire_write_txn.commit().unwrap();

                        // Update the atomic variable `NEXT_EXPIRE_TIME` with the new expire time
                        NEXT_EXPIRE_TIME.store(new_expire_time, Ordering::SeqCst);

                        // Notify the system that it should check for query expirations
                        SHOULD_CHECK_QUERY_EXPIRE.notify_one();
                    }

                    // If there are any album updates collected in the buffer
                    if !buffer.is_empty() {
                        // Send the flattened buffer to the `ALBUM_QUEUE_SENDER`
                        ALBUM_QUEUE_SENDER
                            .get()
                            .unwrap()
                            .send(
                                buffer
                                    .into_iter()
                                    .map(|album_queue| {
                                        if let Some(notify) = album_queue.notify {
                                            notify.notify_one();
                                        }
                                        album_queue.album_list
                                    })
                                    .flatten()
                                    .collect(),
                            )
                            .unwrap();

                        // Log that album updates have been sent to the queue
                        info!("Send queue albums.");
                    }
                })
                .await
                .unwrap(); // Await the completion of the blocking task and unwrap the result
            }
        })
    }
}
