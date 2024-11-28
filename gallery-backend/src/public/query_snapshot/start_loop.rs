use super::QuerySnapshot;
use crate::{
    public::{reduced_data::ReducedData, tree::start_loop::VERSION_COUNT},
    router::get::get_data::Prefetch,
};
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
                let mut result_prefetch_opt = None;
                {
                    if let Some(ref_data) = self.in_memory.iter().next() {
                        expression_hashed_opt = Some(ref_data.key().clone());
                    }
                }
                {
                    if let Some(ref expression) = expression_hashed_opt {
                        let mut ref_mut = self.in_memory.get_mut(expression).unwrap();
                        let ref_data = ref_mut.value_mut();
                        result_prefetch_opt = Some(ref_data.clone());
                    }
                }
                {
                    if let Some(result_prefetch) = result_prefetch_opt {
                        let timer_start = Instant::now();
                        let txn = self.in_disk.begin_write().unwrap();
                        let count_version = &VERSION_COUNT.load(Ordering::Relaxed).to_string();
                        let table_definition: TableDefinition<u64, Option<Prefetch>> =
                            TableDefinition::new(&count_version);
                        {
                            let mut table = txn.open_table(table_definition).unwrap();
                            if let Some(expression_hashed) = expression_hashed_opt.clone() {
                                table.insert(expression_hashed, result_prefetch).unwrap();
                            }
                        }
                        println!("count_version {} table created", count_version);
                        txn.commit().unwrap();
                        info!(duration = &*format!("{:?}", timer_start.elapsed());
                            "Write expression_timetsamp_in_memory cache into disk"
                        );
                    }
                }
                {
                    if let Some(ref expression) = expression_hashed_opt {
                        self.in_memory.remove(expression);
                        info!(
                            "{} items remaining in in-memory cache",
                            self.in_memory.len()
                        );
                    }
                }
            }
            sleep(Duration::from_millis(500));
        });
    }
}
