use super::QuerySnapshot;
use crate::public::{query_snapshot::PrefetchReturn, tree::start_loop::VERSION_COUNT_TIMESTAMP};
use redb::TableDefinition;
use std::{sync::atomic::Ordering, time::Instant};
use tokio::sync::Notify;

pub static SHOULD_FLUSH_QUERY_SNAPSHOT: Notify = Notify::const_new();

impl QuerySnapshot {
    pub fn start_loop(&self) -> tokio::task::JoinHandle<()> {
        tokio::task::spawn(async {
            loop {
                SHOULD_FLUSH_QUERY_SNAPSHOT.notified().await;

                tokio::task::spawn_blocking(|| loop {
                    if self.in_memory.is_empty() {
                        break;
                    }

                    // Narrow scope for the DashMap reference
                    let expression_hashed = {
                        // Attempt to get a reference to one entry:
                        let Some(entry_ref) = self.in_memory.iter().next() else {
                            break;
                        };

                        let expression_hashed = *entry_ref.key();
                        let ref_data = entry_ref.value();

                        // Save to disk
                        let timer_start = Instant::now();
                        let txn = self.in_disk.begin_write().unwrap();
                        let count_version =
                            &VERSION_COUNT_TIMESTAMP.load(Ordering::Relaxed).to_string();
                        let table_definition: TableDefinition<u64, PrefetchReturn> =
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
                    self.in_memory.remove(&expression_hashed);

                    info!(
                        "{} items remaining in in-memory query cache",
                        self.in_memory.len()
                    );
                })
                .await
                .unwrap();
            }
        })
    }
}
