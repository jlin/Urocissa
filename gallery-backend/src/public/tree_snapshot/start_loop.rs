use super::TreeSnapshot;
use crate::public::reduced_data::ReducedData;
use redb::TableDefinition;
use std::{sync::OnceLock, time::Instant};
use tokio::sync::{
    mpsc::{unbounded_channel, UnboundedSender},
    Notify,
};

// Define a global notification for when the tree snapshot should be flushed to disk
pub static SHOULD_FLUSH_TREE_SNAPSHOT: Notify = Notify::const_new();

// Define a global sender for the queue that handles deletion of tree snapshots
pub static TREE_SNAPSHOT_DELETE_QUEUE_SENDER: OnceLock<UnboundedSender<Vec<u128>>> =
    OnceLock::new();

impl TreeSnapshot {
    /// Starts an asynchronous loop that listens for deletion requests and processes them.
    ///
    /// This function spawns a Tokio task that continuously listens for incoming
    /// vectors of timestamps (`u128`) that indicate which tree snapshots should be deleted.
    /// Upon receiving a deletion request, it spawns a blocking task to handle the deletion
    /// of each specified tree snapshot from the on-disk storage.
    ///
    /// # Returns
    ///
    /// A `JoinHandle` for the spawned Tokio task.
    pub fn start_loop_remove(&'static self) -> tokio::task::JoinHandle<()> {
        tokio::task::spawn(async move {
            // Create an unbounded channel for receiving deletion queues
            let (tree_snapshot_delete_queue_sender, mut tree_snapshot_delete_queue_receiver) =
                unbounded_channel::<Vec<u128>>();

            // Initialize the global sender with the sender end of the channel
            TREE_SNAPSHOT_DELETE_QUEUE_SENDER
                .set(tree_snapshot_delete_queue_sender)
                .unwrap();

            // Continuously listen for incoming deletion queues
            while let Some(tree_snapshot_delete_queue) =
                tree_snapshot_delete_queue_receiver.recv().await
            {
                // Spawn a blocking task to process the deletion queue
                tokio::task::spawn_blocking(move || {
                    // Iterate over each timestamp in the deletion queue
                    tree_snapshot_delete_queue
                        .iter()
                        .for_each(|timestamp_delete| {
                            // Begin a write transaction on the disk storage
                            let write_txn = self.in_disk.begin_write().unwrap();

                            // Convert the timestamp to a string to use as the table name
                            let binding = timestamp_delete.to_string();

                            // Define the table structure for deletion
                            let table_definition: TableDefinition<u64, ReducedData> =
                                TableDefinition::new(&binding);

                            // Attempt to delete the specified table
                            match write_txn.delete_table(table_definition) {
                                // If successful, log the deletion
                                Ok(true) => {
                                    info!("Delete tree cache table: {:?}", timestamp_delete)
                                }
                                // If the table was not found, log an error
                                Ok(false) => {
                                    error!(
                                        "Failed to delete tree cache table: {:?}",
                                        timestamp_delete
                                    )
                                }
                                // If an error occurred during deletion, log the error
                                Err(e) => {
                                    error!(
                                        "Failed to delete tree cache table: {:?}, error: {:?}",
                                        timestamp_delete, e
                                    )
                                }
                            }

                            // Log the number of remaining tables in the disk tree cache
                            info!(
                                "{} items remaining in disk tree cache",
                                write_txn.list_tables().unwrap().count()
                            );

                            // Commit the transaction to finalize the deletion
                            write_txn.commit().unwrap();
                        });
                })
                .await
                .unwrap(); // Await the completion of the blocking task
            }
        })
    }

    /// Starts an asynchronous loop that listens for flush notifications and processes them.
    ///
    /// This function spawns a Tokio task that continuously waits for a notification indicating
    /// that the in-memory tree snapshots should be flushed to disk. Upon receiving such a notification,
    /// it spawns a blocking task that iterates over the in-memory snapshots, writes each one to disk,
    /// and removes it from the in-memory cache.
    ///
    /// # Returns
    ///
    /// A `JoinHandle` for the spawned Tokio task.
    pub fn start_loop_flush(&'static self) -> tokio::task::JoinHandle<()> {
        tokio::task::spawn(async {
            loop {
                // Await a notification to flush the tree snapshot
                SHOULD_FLUSH_TREE_SNAPSHOT.notified().await;

                // Spawn a blocking task to handle the flushing process
                tokio::task::spawn_blocking(|| loop {
                    // Check if there are any items in the in-memory cache to flush
                    if self.in_memory.len() > 0 {
                        let mut timestamp_opt = None;
                        let mut data_vec_opt = None;

                        {
                            // Attempt to retrieve the first item from the in-memory cache
                            if let Some(ref_data) = self.in_memory.iter().next() {
                                timestamp_opt = Some(*ref_data.key());
                                data_vec_opt = Some(ref_data.clone());
                            }
                        }

                        // If a timestamp and corresponding data vector are found
                        if let Some(timestamp) = timestamp_opt {
                            if let Some(data_vec) = data_vec_opt {
                                // Convert the timestamp to a string for table naming
                                let timestamp_string = timestamp.to_string();

                                // Record the start time for logging purposes
                                let timer_start = Instant::now();

                                // Begin a write transaction on the disk storage
                                let txn = self.in_disk.begin_write().unwrap();

                                // Define the table structure for the snapshot
                                let table_definition: TableDefinition<u64, ReducedData> =
                                    TableDefinition::new(&timestamp_string);

                                {
                                    // Open the table for writing
                                    let mut table = txn.open_table(table_definition).unwrap();

                                    // Insert each data item into the table with an incremental index
                                    data_vec.iter().enumerate().for_each(|(index, data)| {
                                        table.insert(index as u64, data).unwrap();
                                    })
                                }

                                // Commit the transaction to finalize the write
                                txn.commit().unwrap();

                                // Log the duration taken to write the in-memory cache to disk
                                info!(duration = &*format!("{:?}", timer_start.elapsed());
                                    "Write in-memory cache into disk"
                                );

                                {
                                    // Remove the flushed snapshot from the in-memory cache
                                    self.in_memory.remove(&timestamp_opt.unwrap());

                                    // Log the number of remaining items in the in-memory cache
                                    info!(
                                        "{} items remaining in in-memory tree cache",
                                        self.in_memory.len()
                                    );
                                }
                            }
                        }
                    } else {
                        // If there are no more items to flush, exit the blocking loop
                        break;
                    }
                })
                .await
                .unwrap(); // Await the completion of the blocking task
            }
        })
    }
}
