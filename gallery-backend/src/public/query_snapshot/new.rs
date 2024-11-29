use dashmap::DashMap;
use std::sync::LazyLock;

use super::{PrefetchReturn, QuerySnapshot};

static QUERY_SNAPSHOT_IN_DISK: LazyLock<redb::Database> =
    LazyLock::new(|| redb::Database::create("./db/cache_db.redb").unwrap());

static QUERY_SNAPSHOT_IN_MEMORY: LazyLock<DashMap<u64, PrefetchReturn>> =
    LazyLock::new(|| DashMap::new());

impl QuerySnapshot {
    pub fn new() -> Self {
        Self {
            in_disk: &QUERY_SNAPSHOT_IN_DISK,
            in_memory: &QUERY_SNAPSHOT_IN_MEMORY,
        }
    }
}
