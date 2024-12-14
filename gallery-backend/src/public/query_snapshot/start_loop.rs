use super::QuerySnapshot;
use crate::public::{query_snapshot::PrefetchReturn, tree::start_loop::VERSION_COUNT_TIMESTAMP};
use redb::TableDefinition;
use std::{sync::atomic::Ordering, time::Instant};
use tokio::sync::Notify;

pub static SHOULD_FLUSH_QUERY_SNAPSHOT: Notify = Notify::const_new();

// Implementation block for the `QuerySnapshot` struct
impl QuerySnapshot {
    /// Starts an asynchronous loop that listens for flush notifications and processes query snapshots.
    ///
    /// This function spawns a Tokio task that continuously waits for a `SHOULD_FLUSH_QUERY_SNAPSHOT` notification.
    /// Upon receiving the notification, it spawns a blocking task to flush in-memory query snapshots to disk.
    /// Each snapshot is written to a table named after the current `VERSION_COUNT_TIMESTAMP`, and the in-memory cache is updated accordingly.
    ///
    /// # Returns
    ///
    /// A `JoinHandle` for the spawned Tokio task.
    pub fn start_loop(&self) -> tokio::task::JoinHandle<()> {
        // Spawn an asynchronous Tokio task
        tokio::task::spawn(async {
            // Enter an infinite loop to continuously listen for flush notifications
            loop {
                // Await a notification indicating that the query snapshot should be flushed
                SHOULD_FLUSH_QUERY_SNAPSHOT.notified().await;

                // Spawn a blocking task to handle the flushing process
                tokio::task::spawn_blocking(|| loop {
                    // Check if there are any items in the in-memory query snapshot cache to flush
                    if self.in_memory.len() > 0 {
                        let mut expression_hashed_opt = None;
                        let mut ref_data_opt = None;

                        {
                            // Attempt to retrieve the first item from the in-memory cache
                            if let Some(ref_data) = self.in_memory.iter().next() {
                                // Clone the key (expression hashed) and value (reference data) for processing
                                expression_hashed_opt = Some(ref_data.key().clone());
                                ref_data_opt = Some(ref_data.value().clone());
                            }
                        }

                        // If both the expression hashed and reference data are available, proceed to flush
                        if let Some(expression_hashed) = expression_hashed_opt {
                            if let Some(ref_data) = ref_data_opt {
                                {
                                    // Record the start time for logging the duration of the flush operation
                                    let timer_start = Instant::now();

                                    // Begin a write transaction on the in-disk data store
                                    let txn = self.in_disk.begin_write().unwrap();

                                    // Load the current version count and convert it to a string for table naming
                                    let count_version = &VERSION_COUNT_TIMESTAMP
                                        .load(Ordering::Relaxed)
                                        .to_string();

                                    // Define the table structure using the current version count as the table name
                                    let table_definition: TableDefinition<u64, PrefetchReturn> =
                                        TableDefinition::new(&count_version);

                                    {
                                        // Open the table corresponding to the current version for writing
                                        let mut table = txn.open_table(table_definition).unwrap();

                                        // Insert the reference data into the table with the expression hashed as the key
                                        table.insert(expression_hashed, ref_data).unwrap();
                                    }

                                    // Commit the write transaction to finalize the insertion
                                    txn.commit().unwrap();

                                    // Log the duration taken to write the query cache to disk
                                    info!(duration = &*format!("{:?}", timer_start.elapsed());
                                        "Write query cache into disk"
                                    );
                                }

                                {
                                    // Remove the flushed query snapshot from the in-memory cache
                                    self.in_memory.remove(&expression_hashed);

                                    // Log the number of remaining items in the in-memory query cache
                                    info!(
                                        "{} items remaining in in-memory query cache",
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
                .unwrap(); // Await the completion of the blocking task and unwrap the result
            }
        })
    }
}
