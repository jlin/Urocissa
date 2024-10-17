use super::TreeSnapshot;
use crate::public::tree_snap_shot_in_memory::ReducedData;
use chrono::Utc;
use redb::{TableDefinition, TableHandle};
use std::{
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
                        println!(
                            "Step: Inserting into TREE_SNAPSHOT_IN_DISK {:?}",
                            timer_start.elapsed()
                        );
                    }
                }
                {
                    self.in_memory.remove(&timestamp);
                    println!("TREE_SNAPSHOT_IN_MEMORY has len {}", self.in_memory.len());
                }
            }
            sleep(Duration::from_millis(500));
        });
        tokio::task::spawn_blocking(|| loop {
            let txn = self.in_disk.begin_read().unwrap();
            txn.list_tables().unwrap().for_each(|table_handle| {
                let timestamp = table_handle.name().parse::<u64>().unwrap();
                let current_time_millis = Utc::now().timestamp_millis() as u64;
                let duration_since = current_time_millis - timestamp;
                if duration_since > 1 * 60 * 60 * 1000 {
                    // 1 hours in milliseconds
                    let write_txn = self.in_disk.begin_write().unwrap();

                    match write_txn.delete_table(table_handle) {
                        Ok(true) => println!("Deleted table: {:?}", timestamp),
                        Ok(false) => println!("Failed to delete table: {:?}", timestamp),
                        Err(e) => {
                            eprintln!("Error deleting table: {:?}, error: {:?}", timestamp, e)
                        }
                    }
                    write_txn.commit().unwrap();
                }
            });
            sleep(Duration::from_millis(500));
        });
    }
}
