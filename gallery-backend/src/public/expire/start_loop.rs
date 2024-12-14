use super::Expire;
use crate::public::{
    query_snapshot::{PrefetchReturn, QUERY_SNAPSHOT},
    tree::start_loop::VERSION_COUNT_TIMESTAMP,
    tree_snapshot::start_loop::TREE_SNAPSHOT_DELETE_QUEUE_SENDER,
    utils::get_current_timestamp_u64,
};
use rayon::iter::{ ParallelBridge, ParallelIterator};
use redb::{ReadableTable, TableDefinition, TableHandle};
use std::{
    sync::atomic::{AtomicU64, Ordering},
    time::Duration,
};
use tokio::{sync::Notify, time::sleep};

pub static SHOULD_CHECK_QUERY_EXPIRE: Notify = Notify::const_new();
pub static NEXT_EXPIRE_TIME: AtomicU64 = AtomicU64::new(0);

// Implementation block for the `Expire` struct
impl Expire {
    /// Starts an asynchronous loop that handles the expiration of query snapshots.
    ///
    /// This function spawns a Tokio task that continuously performs the following:
    /// 1. Spawns a blocking task to process and delete expired query snapshot tables.
    /// 2. Determines the next expiration time and sleeps until that time or until a new expiration is scheduled.
    ///
    /// # Returns
    ///
    /// A `JoinHandle` for the spawned Tokio task.
    pub fn start_loop(&'static self) -> tokio::task::JoinHandle<()> {
        // Spawn an asynchronous Tokio task
        tokio::task::spawn(async {
            // Enter an infinite loop to continuously handle expirations
            loop {
                // Spawn a blocking task to process expiration logic
                tokio::task::spawn_blocking(|| {
                    // Begin a write transaction on the `QUERY_SNAPSHOT` in-disk data store
                    let write_txn = QUERY_SNAPSHOT.in_disk.begin_write().unwrap();

                    // List all tables in the `QUERY_SNAPSHOT` data store
                    write_txn
                        .list_tables()
                        .unwrap()
                        // Convert the iterator to a parallel iterator using `par_bridge` for concurrent processing
                        .par_bridge()
                        // Iterate over each table handle concurrently
                        .for_each(|table_handle| {
                            // Attempt to parse the table name as a `u64` timestamp
                            if let Ok(timestamp) = table_handle.name().parse::<u64>() {
                                // Check if the current `VERSION_COUNT_TIMESTAMP` is greater than the table's timestamp
                                // and if the table is expired based on custom logic
                                if VERSION_COUNT_TIMESTAMP.load(Ordering::Relaxed) > timestamp
                                    && self.expired_check(timestamp)
                                {
                                    // Convert the timestamp to a string to define the table name
                                    let binding = timestamp.to_string();
                                    
                                    // Define the structure of the table using the timestamp as the name
                                    let table_definition: TableDefinition<u64, PrefetchReturn> =
                                        TableDefinition::new(&binding);
                                    
                                    // Begin a read transaction to access the table
                                    let read_txn = QUERY_SNAPSHOT.in_disk.begin_read().unwrap();
                                    
                                    // Open the specific table corresponding to the timestamp
                                    let table = read_txn.open_table(table_definition).unwrap();
                                    
                                    // Collect all timestamps from the `PrefetchReturn` entries in the table
                                    let tree_snapshot_delete_queue: Vec<_> = table
                                        .iter()
                                        .unwrap()
                                        .par_bridge()
                                        .filter_map(|result| {
                                            let (_, value) = result.unwrap();
                                            let prefetch_return = value.value();
                                            // Extract the timestamp if `prefetch_return` is `Some`
                                            prefetch_return.map(|prefetch| prefetch.timestamp)
                                        })
                                        .collect();

                                    // Attempt to delete the table corresponding to the expired timestamp
                                    match write_txn.delete_table(table_handle) {
                                        // If deletion is successful, log the event and send the delete queue
                                        Ok(true) => {
                                            info!("Delete query cache table: {:?}", timestamp);
                                            TREE_SNAPSHOT_DELETE_QUEUE_SENDER
                                                .get()
                                                .unwrap()
                                                .send(tree_snapshot_delete_queue)
                                                .unwrap();
                                        }
                                        // If the table was not found or already deleted, log an error
                                        Ok(false) => {
                                            error!(
                                                "Failed to delete query cache table: {:?}",
                                                timestamp
                                            )
                                        }
                                        // If an error occurred during deletion, log the error details
                                        Err(e) => {
                                            error!(
                                                "Failed to delete query cache table: {:?}, error: {:?}",
                                                timestamp, e
                                            )
                                        }
                                    }
                                    
                                    // Log the number of remaining tables in the `QUERY_SNAPSHOT` data store
                                    info!(
                                        "{} items remaining in disk query cache",
                                        write_txn.list_tables().unwrap().count()
                                    );
                                }
                            }
                        });
                    
                    // Commit the write transaction to finalize any deletions performed
                    write_txn.commit().unwrap();
                })
                .await
                .unwrap(); // Await the completion of the blocking task and unwrap the result
                
                // Load the next scheduled expiration time from the atomic variable
                let expire_time = NEXT_EXPIRE_TIME.load(Ordering::Relaxed);
                
                // Get the current timestamp
                let current_time = get_current_timestamp_u64();
                
                // Determine if the next expiration time is in the future
                if expire_time > current_time {
                    // Calculate the duration to sleep until the next expiration
                    let sleep_duration = expire_time - current_time;
                    let duration = Duration::from_millis(sleep_duration);
                    
                    // Log the duration for which the expire thread will sleep
                    info!("Expire thread sleep {:?}", duration);
                    
                    // Sleep asynchronously for the calculated duration
                    sleep(duration).await;
                } else {
                    // If the next expiration time is not in the future, wait until notified
                    info!("Expire thread sleep until notified.");
                    SHOULD_CHECK_QUERY_EXPIRE.notified().await
                }
            }
        })
    }
}
