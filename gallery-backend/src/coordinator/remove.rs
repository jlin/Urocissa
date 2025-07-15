use redb::TableDefinition;

use crate::{db::tree_snapshot::TREE_SNAPSHOT, structure::reduced_data::ReducedData};

/// Removes a tree cache table by its timestamp.
pub fn remove_task(timestamp: u128) -> anyhow::Result<()> {
    let write_txn = TREE_SNAPSHOT.in_disk.begin_write().unwrap();
    let binding = timestamp.to_string();
    let table_definition: TableDefinition<u64, ReducedData> = TableDefinition::new(&binding);

    match write_txn.delete_table(table_definition) {
        Ok(true) => {
            info!("Delete tree cache table: {:?}", timestamp)
        }
        Ok(false) => {
            error!("Failed to delete tree cache table: {:?}", timestamp)
        }
        Err(err) => {
            error!(
                "Failed to delete tree cache table: {:?}, error: {:#?}",
                timestamp, err
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
