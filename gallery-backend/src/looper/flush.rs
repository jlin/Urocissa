use crate::looper::tree_snapshot::TREE_SNAPSHOT;
use crate::structure::reduced_data::ReducedData;
use redb::TableDefinition;
use std::time::Instant;

pub fn flush_task() -> anyhow::Result<()> {
    loop {
        if TREE_SNAPSHOT.in_memory.is_empty() {
            break;
        }

        // Narrow scope for the DashMap reference
        let timestamp = {
            // Attempt to get a reference to one entry:
            let Some(entry_ref) = TREE_SNAPSHOT.in_memory.iter().next() else {
                break;
            };

            let timestamp = *entry_ref.key();
            let timestamp_str = timestamp.to_string();

            let timer_start = Instant::now();
            let txn = TREE_SNAPSHOT.in_disk.begin_write().unwrap();
            let table_definition: TableDefinition<u64, ReducedData> =
                TableDefinition::new(&timestamp_str);

            {
                let mut table = txn.open_table(table_definition).unwrap();
                for (index, data) in entry_ref.iter().enumerate() {
                    table.insert(index as u64, data).unwrap();
                }
            }

            txn.commit().unwrap();

            info!(
                duration = &*format!("{:?}", timer_start.elapsed());
                "Write in-memory cache into disk"
            );
            timestamp
        };

        //Remove from DashMap *after* reference is dropped
        TREE_SNAPSHOT.in_memory.remove(&timestamp);
        info!(
            "{} items remaining in in-memory tree cache",
            TREE_SNAPSHOT.in_memory.len()
        );
    }
    Ok(())
}
