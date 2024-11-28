use dashmap::DashMap;
use std::sync::LazyLock;

use crate::{public::reduced_data::ReducedData, router::get::get_data::Prefetch};

use super::TreeSnapshot;

static TREE_SNAPSHOT_IN_DISK: LazyLock<redb::Database> =
    LazyLock::new(|| redb::Database::create("./db/temp_db.redb").unwrap());

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
