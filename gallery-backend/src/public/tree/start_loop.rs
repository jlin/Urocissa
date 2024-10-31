use super::Tree;
use crate::public::database_struct::database_timestamp::DataBaseTimestamp;
use crate::public::redb::DATA_TABLE;
use log::warn;
use rayon::prelude::ParallelSliceMut;
use redb::ReadableTable;
use std::sync::atomic::AtomicBool;
use std::thread::sleep;
use std::{sync::atomic::Ordering, time::Duration};

pub static SHOULD_RESET: AtomicBool = AtomicBool::new(false);

impl Tree {
    pub fn start_loop(&self) {
        tokio::task::spawn_blocking(|| loop {
            if SHOULD_RESET.swap(false, Ordering::SeqCst) {
                let table = self
                    .in_disk
                    .begin_read()
                    .unwrap()
                    .open_table(DATA_TABLE)
                    .unwrap();
                let priority_list = vec!["DateTimeOriginal", "filename", "modified", "scan_time"];
                let mut data_vec: Vec<DataBaseTimestamp> = table
                    .iter()
                    .unwrap()
                    .map(|guard| {
                        let (_key, value) = guard.unwrap();
                        let database = value.value();
                        DataBaseTimestamp::new(database, &priority_list)
                    })
                    .collect();
                data_vec.par_sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
                *self.in_memory.write().unwrap() = data_vec;
                info!("In-memory cache updated.");
            }
            sleep(Duration::from_millis(500))
        });
    }
}
