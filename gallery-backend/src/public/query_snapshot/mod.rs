pub mod new;
pub mod read_query_snapshots;
pub mod start_loop;
use crate::{public::reduced_data::ReducedData, router::get::get_data::Prefetch};
use dashmap::DashMap;
use std::sync::LazyLock;

#[derive(Debug)]
pub struct QuerySnapshot {
    pub in_disk: &'static redb::Database,
    pub in_memory: &'static DashMap<u64, Option<Prefetch>>,
}

pub static QUERY_SNAPSHOT: LazyLock<QuerySnapshot> = LazyLock::new(|| {
    let new = QuerySnapshot::new();
    new.start_loop();
    new
});
