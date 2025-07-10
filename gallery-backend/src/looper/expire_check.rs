use crate::{
    coordinator::{COORDINATOR, Task, remove::RemoveTask},
    db::{expire::EXPIRE, query_snapshot::QUERY_SNAPSHOT, tree::VERSION_COUNT_TIMESTAMP},
    router::get::get_prefetch::Prefetch,
};
use rayon::iter::{ParallelBridge, ParallelIterator};
use redb::{ReadableTable, TableDefinition, TableHandle};
use std::sync::atomic::Ordering;

pub fn expire_check_task() -> anyhow::Result<()> {
    let write_txn = QUERY_SNAPSHOT.in_disk.begin_write().unwrap();
    // Iter over all tables in QUERY_SNAPSHOT
    write_txn
        .list_tables()
        .unwrap()
        .par_bridge()
        .for_each(|table_handle| {
            if let Ok(timestamp) = table_handle.name().parse::<u64>()
                && VERSION_COUNT_TIMESTAMP.load(Ordering::Relaxed) > timestamp
                && EXPIRE.expired_check(timestamp)
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
    Ok(())
}
