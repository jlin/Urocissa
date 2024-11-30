use super::QuerySnapshot;
use crate::public::{query_snapshot::PrefetchReturn, tree::start_loop::VERSION_COUNT};
use redb::TableDefinition;
use std::{sync::atomic::Ordering, time::Instant};
use tokio::sync::Notify;

pub static SHOULD_FLUSH_QUERY_SNAPSHOT: Notify = Notify::const_new();

impl QuerySnapshot {
    pub(super) fn start_loop(&self) -> tokio::task::JoinHandle<()> {
        tokio::task::spawn(async {
            loop {
                SHOULD_FLUSH_QUERY_SNAPSHOT.notified().await;
                tokio::task::spawn_blocking(|| loop {
                    if self.in_memory.len() > 0 {
                        let mut expression_hashed_opt = None;
                        let mut ref_data_opt = None;
                        {
                            if let Some(ref_data) = self.in_memory.iter().next() {
                                expression_hashed_opt = Some(ref_data.key().clone());
                                ref_data_opt = Some(ref_data.value().clone());
                            }
                        }

                        if let Some(expression_hashed) = expression_hashed_opt {
                            if let Some(ref_data) = ref_data_opt {
                                {
                                    let timer_start = Instant::now();
                                    let txn = self.in_disk.begin_write().unwrap();
                                    let count_version =
                                        &VERSION_COUNT.load(Ordering::Relaxed).to_string();
                                    let table_definition: TableDefinition<u64, PrefetchReturn> =
                                        TableDefinition::new(&count_version);
                                    {
                                        let mut table = txn.open_table(table_definition).unwrap();

                                        table.insert(expression_hashed, ref_data).unwrap();
                                    }
                                    txn.commit().unwrap();
                                    info!(duration = &*format!("{:?}", timer_start.elapsed());
                                        "Write query cache into disk"
                                    );
                                }
                                {
                                    self.in_memory.remove(&expression_hashed);
                                    info!(
                                        "{} items remaining in in-memory query cache",
                                        self.in_memory.len()
                                    );
                                }
                            }
                        }
                    } else {
                        break;
                    }
                })
                .await
                .unwrap();
            }
        })
    }
}
