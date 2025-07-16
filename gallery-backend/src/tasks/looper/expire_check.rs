use crate::{
    tasks::{COORDINATOR, actor::remove::RemoveTask},
    public::db::{expire::EXPIRE, query_snapshot::QUERY_SNAPSHOT, tree::VERSION_COUNT_TIMESTAMP},
    router::get::get_prefetch::Prefetch,
};
use rayon::iter::{ParallelBridge, ParallelIterator};
use redb::{ReadableTable, TableDefinition, TableHandle};
use std::sync::atomic::Ordering;
pub fn expire_check_task() -> anyhow::Result<()> {
    let write_txn = QUERY_SNAPSHOT.in_disk.begin_write().unwrap();

    write_txn
        .list_tables()
        .unwrap()
        .par_bridge()
        .try_for_each::<_, anyhow::Result<()>>(|table_handle| {
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
            Ok(())
        })?;

    write_txn.commit().unwrap();
    Ok(())
}
