use super::Expire;
use crate::{
    constant::SNAPSHOT_MAX_LIFETIME_MS,
    coordinator::{COORDINATOR, Task, remove::RemoveTask},
    looper::{query_snapshot::QUERY_SNAPSHOT, tree::VERSION_COUNT_TIMESTAMP},
    router::get::get_prefetch::Prefetch,
    utils::start_loop_util,
};
use rayon::iter::{ParallelBridge, ParallelIterator};
use redb::{ReadableTable, TableDefinition, TableHandle};
use std::{
    sync::{Arc, OnceLock, atomic::Ordering},
    time::Duration,
};
use tokio::sync::{Notify, mpsc::UnboundedSender};

static EXPIRE_CHECK_SENDER: OnceLock<UnboundedSender<Option<Arc<Notify>>>> = OnceLock::new();

impl Expire {
    pub fn start_loop(&'static self) -> tokio::task::JoinHandle<()> {
        start_loop_util(
            Some((Duration::from_millis(SNAPSHOT_MAX_LIFETIME_MS), None)),
            &EXPIRE_CHECK_SENDER,
            |buffer| {
                let write_txn = QUERY_SNAPSHOT.in_disk.begin_write().unwrap();
                // Iter over all tables in QUERY_SNAPSHOT
                write_txn
                    .list_tables()
                    .unwrap()
                    .par_bridge()
                    .for_each(|table_handle| {
                        if let Ok(timestamp) = table_handle.name().parse::<u64>()
                            && VERSION_COUNT_TIMESTAMP.load(Ordering::Relaxed) > timestamp
                            && self.expired_check(timestamp)
                        {
                            // the table in QUERY_SNAPSHOT expired
                            // perform purge
                            let binding = timestamp.to_string();
                            let table_definition: TableDefinition<u64, Prefetch> =
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
                                        .map(|result| {
                                            let (_, guard) = result.unwrap();
                                            let prefetch_return = guard.value();
                                            prefetch_return.timestamp
                                        })
                                        .collect();

                                    tree_snapshot_delete_queue.iter().for_each(|timestamp| {
                                        COORDINATOR
                                            .submit(Task::Remove(RemoveTask::new(*timestamp)))
                                            .unwrap();
                                    });
                                }
                                Ok(false) => {
                                    error!("Failed to delete query cache table: {:?}", timestamp);
                                }
                                Err(e) => {
                                    error!(
                                        "Failed to delete query cache table: {:?}, error: {:#?}",
                                        timestamp, e
                                    );
                                }
                            }

                            info!(
                                "{} items remaining in disk query cache",
                                write_txn.list_tables().unwrap().count()
                            );
                        }
                    });
                write_txn.commit().unwrap();
                buffer.iter().for_each(|notify_opt| {
                    if let Some(notify) = notify_opt {
                        notify.notify_one()
                    }
                });
            },
        )
    }
    pub fn expire_check(&self) {
        EXPIRE_CHECK_SENDER.get().unwrap().send(None).unwrap();
    }
    pub async fn _expire_check_async(&self) {
        let notify = Arc::new(Notify::new());
        EXPIRE_CHECK_SENDER
            .get()
            .unwrap()
            .send(Some(notify.clone()))
            .unwrap();
        notify.notified().await
    }
}
