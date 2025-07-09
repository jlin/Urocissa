use redb::TableDefinition;

use crate::{looper::tree_snapshot::TREE_SNAPSHOT, structure::reduced_data::ReducedData};

#[derive(Debug)]
pub struct RemoveTask {
    pub timestamp: u128,
}
impl RemoveTask {
    pub fn new(timestamp: u128) -> Self {
        Self { timestamp }
    }
}

pub fn remove_task(task: RemoveTask) -> anyhow::Result<()> {
    let timestamp_delete = task.timestamp;
    let write_txn = TREE_SNAPSHOT.in_disk.begin_write().unwrap();
    let binding = timestamp_delete.to_string();
    let table_definition: TableDefinition<u64, ReducedData> = TableDefinition::new(&binding);

    match write_txn.delete_table(table_definition) {
        Ok(true) => {
            info!("Delete tree cache table: {:?}", timestamp_delete)
        }
        Ok(false) => {
            error!("Failed to delete tree cache table: {:?}", timestamp_delete)
        }
        Err(e) => {
            error!(
                "Failed to delete tree cache table: {:?}, error: {:#?}",
                timestamp_delete, e
            )
        }
    }

    info!(
        "{} items remaining in disk tree cache",
        write_txn.list_tables().unwrap().count()
    );

    write_txn.commit().unwrap();
    Ok(())
}
