use dashmap::DashMap;
use std::sync::LazyLock;

use crate::{public::reduced_data::ReducedData, router::get::get_data::Prefetch};

use super::QuerySnapshot;

static QUERY_SNAPSHOT_IN_DISK: LazyLock<redb::Database> =
    LazyLock::new(|| redb::Database::create("./db/cache_db.redb").unwrap());

static QUERY_SNAPSHOT_IN_MEMORY: LazyLock<DashMap<u64, Option<Prefetch>>> =
    LazyLock::new(|| DashMap::new());

impl QuerySnapshot {
    pub fn new() -> Self {
        Self {
            in_disk: &QUERY_SNAPSHOT_IN_DISK,
            in_memory: &QUERY_SNAPSHOT_IN_MEMORY,
        }
    }
}
