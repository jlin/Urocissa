pub mod new;
pub mod read_query_snapshots;
pub mod start_loop;
use crate::router::get::get_data::Prefetch;
use dashmap::DashMap;
use std::sync::LazyLock;

pub type PrefetchReturn = Option<Prefetch>;

#[derive(Debug)]
pub struct QuerySnapshot {
    pub in_disk: &'static redb::Database,
    pub in_memory: &'static DashMap<u64, PrefetchReturn>, // query_hash -> prefetch
}

pub static QUERY_SNAPSHOT: LazyLock<QuerySnapshot> = LazyLock::new(|| {
    let new = QuerySnapshot::new();
    new.start_loop();
    new
});
