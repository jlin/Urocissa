use dashmap::DashMap;
use std::{fs, sync::LazyLock};

use crate::public::reduced_data::ReducedData;

use super::TreeSnapshot;

static TREE_SNAPSHOT_IN_DISK: LazyLock<redb::Database> = LazyLock::new(|| {
    let db_path = "./db/temp_db.redb";
    if fs::metadata(db_path).is_ok() {
        match fs::remove_file(db_path) {
            Ok(_) => {
                info!("Clear cache");
            }
            Err(_) => {
                error!("Fail to delete cache data ./db/temp_db.redb")
            }
        }
    }
    redb::Database::create(db_path).unwrap()
});

static TREE_SNAPSHOT_IN_MEMORY: LazyLock<DashMap<String, Vec<ReducedData>>> =
    LazyLock::new(|| DashMap::new());

impl TreeSnapshot {
    pub fn new() -> Self {
        Self {
            in_disk: &TREE_SNAPSHOT_IN_DISK,
            in_memory: &TREE_SNAPSHOT_IN_MEMORY,
        }
    }
}
