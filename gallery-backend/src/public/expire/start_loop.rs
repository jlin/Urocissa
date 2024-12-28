use super::Expire;
use crate::public::{
    query_snapshot::{PrefetchReturn, QUERY_SNAPSHOT},
    tree::start_loop::VERSION_COUNT_TIMESTAMP,
    tree_snapshot::start_loop::TREE_SNAPSHOT_DELETE_QUEUE_SENDER,
    utils::get_current_timestamp_u64,
};
use rayon::iter::{ParallelBridge, ParallelIterator};
use redb::{ReadableTable, TableDefinition, TableHandle};
use std::{
    sync::atomic::{AtomicU64, Ordering},
    time::Duration,
};
use tokio::{sync::Notify, time::sleep};

pub static SHOULD_CHECK_QUERY_EXPIRE: Notify = Notify::const_new();
pub static NEXT_EXPIRE_TIME: AtomicU64 = AtomicU64::new(0);

impl Expire {
    pub fn start_loop(&'static self) -> tokio::task::JoinHandle<()> {
        tokio::task::spawn(async {
            loop {
                tokio::task::spawn_blocking(|| {
                    let write_txn = QUERY_SNAPSHOT.in_disk.begin_write().unwrap();
                    // Iter over all tables in QUERY_SNAPSHOT
                    write_txn
                        .list_tables()
                        .unwrap()
                        .par_bridge()
                        .for_each(|table_handle| {
                            if let Ok(timestamp) = table_handle.name().parse::<u64>() {
                                if VERSION_COUNT_TIMESTAMP.load(Ordering::Relaxed) > timestamp
                                    && self.expired_check(timestamp)
                                {
                                    // the table in QUERY_SNAPSHOT expired
                                    // perform purge
                                    let binding = timestamp.to_string();
                                    let table_definition: TableDefinition<u64, PrefetchReturn> =
                                        TableDefinition::new(&binding);

                                    let read_txn = QUERY_SNAPSHOT.in_disk.begin_read().unwrap();
                                    let table = read_txn.open_table(table_definition).unwrap();

                                    match write_txn.delete_table(table_handle) {
                                        Ok(true) => {
                                            info!("Delete query cache table: {:?}", timestamp);
                                            // QUERY_SNAPSHOT purge is complete
                                            // TREE_SNAPSHOT is no longer needed
                                            let tree_snapshot_delete_queue: Vec<_> = table
                                            .iter()
                                            .unwrap()
                                            .par_bridge()
                                            .filter_map(|result| {
                                                let (_, value) = result.unwrap();
                                                value.value().map(|prefetch| prefetch.timestamp)
                                            })
                                            .collect();
                                        
                                            TREE_SNAPSHOT_DELETE_QUEUE_SENDER
                                                .get()
                                                .unwrap()
                                                .send(tree_snapshot_delete_queue)
                                                .unwrap();
                                        }
                                        Ok(false) => {
                                            error!("Failed to delete query cache table: {:?}", timestamp);
                                        }
                                        Err(e) => {
                                            error!(
                                                "Failed to delete query cache table: {:?}, error: {:?}",
                                                timestamp,
                                                e
                                            );
                                        }
                                    }

                                    info!(
                                        "{} items remaining in disk query cache",
                                        write_txn.list_tables().unwrap().count()
                                    );
                                }
                            }
                        });
                    write_txn.commit().unwrap();
                })
                .await
                .unwrap();

                let expire_time = NEXT_EXPIRE_TIME.load(Ordering::Relaxed);
                let current_time = get_current_timestamp_u64();

                if expire_time > current_time {
                    let sleep_duration = expire_time - current_time;
                    let duration = Duration::from_millis(sleep_duration);
                    info!("Expire thread sleep {:?}", duration);
                    sleep(duration).await;
                } else {
                    info!("Expire thread sleep until notified.");
                    SHOULD_CHECK_QUERY_EXPIRE.notified().await
                }
            }
        })
    }
}
