use crate::public::db::expire::EXPIRE;
use crate::public::db::query_snapshot::QUERY_SNAPSHOT;
use crate::public::db::tree::VERSION_COUNT_TIMESTAMP;
use crate::router::get::get_prefetch::Prefetch;
use crate::tasks::COORDINATOR;
use crate::tasks::actor::remove::RemoveTask;
use crate::tasks::batcher::QueueApi;
use rayon::iter::{ParallelBridge, ParallelIterator};

use redb::{ReadableTable, TableDefinition, TableHandle};
use std::sync::atomic::Ordering;

pub static EXPIRE_CHECK_QUEUE: QueueApi<()> = QueueApi::new(update_tree_task);

pub fn update_tree_task(_: Vec<()>) {
    let write_txn = QUERY_SNAPSHOT.in_disk.begin_write().unwrap();

    write_txn
        .list_tables()
        .unwrap()
        .par_bridge()
        .for_each(|table_handle| {
            if let Ok(timestamp) = table_handle.name().parse::<u64>()
                && VERSION_COUNT_TIMESTAMP.load(Ordering::Relaxed) > timestamp
                && EXPIRE.expired_check(timestamp)
            {
                let binding = timestamp.to_string();
                let table_definition: TableDefinition<u64, Prefetch> =
                    TableDefinition::new(&binding);

                let read_txn = QUERY_SNAPSHOT.in_disk.begin_read().unwrap();
                let table = read_txn.open_table(table_definition).unwrap();

                match write_txn.delete_table(table_handle) {
                    Ok(true) => {
                        info!("Delete query cache table: {:?}", timestamp);
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

                        for timestamp in tree_snapshot_delete_queue {
                            let _ = COORDINATOR.execute_detached(RemoveTask::new(timestamp));
                        }
                    }
                    Ok(false) => {
                        error!("Failed to delete query cache table: {:?}", timestamp);
                    }
                    Err(err) => {
                        error!(
                            "Failed to delete query cache table: {:?}, error: {:#?}",
                            timestamp, err
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
}
