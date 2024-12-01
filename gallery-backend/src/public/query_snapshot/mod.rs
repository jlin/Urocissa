pub mod new;
pub mod read_query_snapshots;
pub mod start_loop;
use crate::router::get::get_data::Prefetch;
use dashmap::DashMap;
use redb::TableDefinition;
use std::sync::LazyLock;

pub type PrefetchReturn = Option<Prefetch>;

pub static EXPIRE_TABLE_DEFINITIONF: TableDefinition<u64, Option<u64>> =
    TableDefinition::new("expire_table");

#[derive(Debug)]
pub struct QuerySnapshot {
    pub in_disk: &'static redb::Database,
    pub in_memory: &'static DashMap<u64, PrefetchReturn>, // hash of query and VERSION_COUNT -> prefetch
}

pub static QUERY_SNAPSHOT: LazyLock<QuerySnapshot> = LazyLock::new(|| QuerySnapshot::new());
