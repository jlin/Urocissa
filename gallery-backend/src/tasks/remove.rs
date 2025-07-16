use redb::TableDefinition;
use tokio::task::spawn_blocking;

use crate::{
    public::db::tree_snapshot::TREE_SNAPSHOT, public::structure::reduced_data::ReducedData,
    tasks::actor::Task,
};

pub struct RemoveTask {
    pub timestamp: u128,
}

impl RemoveTask {
    pub fn new(timestamp: u128) -> Self {
        Self { timestamp }
    }
}

impl Task for RemoveTask {
    type Output = anyhow::Result<()>;

    fn run(self) -> impl std::future::Future<Output = Self::Output> + Send {
        async move {
            let result = spawn_blocking(move || remove_task(self.timestamp))
                .await
                .expect("blocking task panicked");
            result
        }
    }
}

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
