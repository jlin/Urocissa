use crate::public::db::query_snapshot::QUERY_SNAPSHOT;
use crate::public::db::tree::VERSION_COUNT_TIMESTAMP;
use crate::router::get::get_prefetch::Prefetch;
use crate::tasks::batcher::QueueApi;
use redb::TableDefinition;
use std::sync::atomic::Ordering;
use std::time::Instant;

pub static FLUSH_QUERY_SNAPSHOT_QUEUE: QueueApi<()> = QueueApi::new(flush_tree_snapshot_task);

fn flush_tree_snapshot_task(_: Vec<()>) {
    loop {
        if QUERY_SNAPSHOT.in_memory.is_empty() {
            break;
        }

        // Narrow scope for the DashMap reference
        let expression_hashed = {
            // Attempt to get a reference to one entry:
            let Some(entry_ref) = QUERY_SNAPSHOT.in_memory.iter().next() else {
                break;
            };

            let expression_hashed = *entry_ref.key();
            let ref_data = entry_ref.value();

            // Save to disk
            let timer_start = Instant::now();
            let txn = QUERY_SNAPSHOT.in_disk.begin_write().unwrap();
            let count_version = &VERSION_COUNT_TIMESTAMP.load(Ordering::Relaxed).to_string();
            let table_definition: TableDefinition<u64, Prefetch> =
                TableDefinition::new(count_version);

            {
                let mut table = txn.open_table(table_definition).unwrap();
                table.insert(expression_hashed, ref_data).unwrap();
            }

            txn.commit().unwrap();
            info!(
                duration = &*format!("{:?}", timer_start.elapsed());
                "Write query cache into disk"
            );

            // Return the hashed key, so we can remove it below
            expression_hashed
        };

        // Remove from DashMap *after* reference is dropped
        QUERY_SNAPSHOT.in_memory.remove(&expression_hashed);

        info!(
            "{} items remaining in in-memory query cache",
            QUERY_SNAPSHOT.in_memory.len()
        );
    }
}
