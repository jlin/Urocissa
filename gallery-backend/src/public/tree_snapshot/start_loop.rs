use super::TreeSnapshot;
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
impl TreeSnapshot {
    pub(super) fn start_loop(&self) {
        tokio::task::spawn_blocking(|| loop {
            if self.in_memory.len() > 0 {
                let mut timestamp = String::new();
                let mut data_vec: Vec<ReducedData> = vec![];
                {
                    if let Some(ref_data) = self.in_memory.iter().next() {
                        timestamp = ref_data.key().clone()
                    }
                }
                {
                    if timestamp.len() != 0 {
                        let mut ref_mut = self.in_memory.get_mut(&timestamp).unwrap();
                        let ref_data = ref_mut.value_mut();
                        data_vec = ref_data.clone();
                    }
                }
                {
                    if data_vec.len() != 0 {
                        let timer_start = Instant::now();
                        let txn = self.in_disk.begin_write().unwrap();
                        let table_definition: TableDefinition<u64, ReducedData> =
                            TableDefinition::new(&timestamp);
                        {
                            let mut table = txn.open_table(table_definition).unwrap();
                            data_vec.iter().enumerate().for_each(|(index, data)| {
                                table.insert(index as u64, data).unwrap();
                            })
                        }
                        txn.commit().unwrap();
                        info!(duration = &*format!("{:?}", timer_start.elapsed());
                            "Write in-memory cache into disk"
                        );
                    }
                }
                {
                    self.in_memory.remove(&timestamp);
                    info!(
                        "{} items remaining in in-memory cache",
                        self.in_memory.len()
                    );
                }
            }
            sleep(Duration::from_millis(500));
        });
        tokio::task::spawn_blocking(|| loop {
            if self.expression_timestamp_in_memory.len() > 0 {
                let mut expression_hashed_opt = None;
                let mut result_prefetch_opt = None;
                {
                    if let Some(ref_data) = self.expression_timestamp_in_memory.iter().next() {
                        expression_hashed_opt = Some(ref_data.key().clone());
                    }
                }
                {
                    if let Some(ref expression) = expression_hashed_opt {
                        let mut ref_mut = self
                            .expression_timestamp_in_memory
                            .get_mut(expression)
                            .unwrap();
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
                        self.expression_timestamp_in_memory.remove(expression);
                        info!(
                            "{} items remaining in in-memory cache",
                            self.in_memory.len()
                        );
                    }
                }
            }
            sleep(Duration::from_millis(500));
        });
        /* tokio::task::spawn_blocking(|| loop {
            let txn = self.in_disk.begin_read().unwrap();
            txn.list_tables().unwrap().for_each(|table_handle| {
                let timestamp = table_handle.name().parse::<u64>().unwrap();
                let current_time_millis = Utc::now().timestamp_millis() as u64;
                let duration_since = current_time_millis - timestamp;
                if duration_since > 1 * 60 * 60 * 1000 {
                    // 1 hours in milliseconds
                    let write_txn = self.in_disk.begin_write().unwrap();

                    match write_txn.delete_table(table_handle) {
                        Ok(true) => info!("Delete table: {:?}", timestamp),
                        Ok(false) => error!("Failed to delete table: {:?}", timestamp),
                        Err(e) => {
                            error!("Failed to delete table: {:?}, error: {:?}", timestamp, e)
                        }
                    }
                    write_txn.commit().unwrap();
                }
            });
            sleep(Duration::from_millis(500));
        }); */
    }
}
