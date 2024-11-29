use super::QuerySnapshot;
use crate::public::{query_snapshot::PrefetchReturn, tree::start_loop::VERSION_COUNT};
use redb::TableDefinition;
use std::{
    sync::atomic::Ordering,
    thread::sleep,
    time::{Duration, Instant},
};

impl QuerySnapshot {
    pub(super) fn start_loop(&self) {
        tokio::task::spawn_blocking(|| loop {
            if self.in_memory.len() > 0 {
                let mut expression_hashed_opt = None;
                {
                    if let Some(ref_data) = self.in_memory.iter().next() {
                        expression_hashed_opt = Some(ref_data.key().clone());
                    }
                }
                {
                    if let Some(ref expression) = expression_hashed_opt {
                        {
                            let mut ref_mut = self.in_memory.get_mut(expression).unwrap();
                            let ref_data = ref_mut.value_mut();

                            let timer_start = Instant::now();
                            let txn = self.in_disk.begin_write().unwrap();
                            let count_version = &VERSION_COUNT.load(Ordering::Relaxed).to_string();
                            let table_definition: TableDefinition<u64, PrefetchReturn> =
                                TableDefinition::new(&count_version);
                            {
                                let mut table = txn.open_table(table_definition).unwrap();
                                if let Some(expression_hashed) = expression_hashed_opt.clone() {
                                    table.insert(expression_hashed, ref_data).unwrap();
                                }
                            }
                            println!("count_version {} table created", count_version);
                            txn.commit().unwrap();
                            info!(duration = &*format!("{:?}", timer_start.elapsed());
                                "Write query cache into disk"
                            );
                        }
                        {
                            if let Some(ref expression) = expression_hashed_opt {
                                self.in_memory.remove(expression);
                                info!(
                                    "{} items remaining in in-memory query cache",
                                    self.in_memory.len()
                                );
                            }
                        }
                    }
                }
            }
            sleep(Duration::from_millis(500));
        });
    }
}
