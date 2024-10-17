pub mod new;
pub mod read_rows;
pub mod read_scrollbar;
pub mod read_tags;
pub mod read_tree_snapshot;
pub mod start_loop;

use std::sync::LazyLock;

use dashmap::DashMap;

use crate::public::tree_snap_shot_in_memory::ReducedData;

#[derive(Debug)]
pub struct TreeSnapshot {
    pub in_disk: &'static redb::Database,
    pub in_memory: &'static DashMap<String, Vec<ReducedData>>,
}

pub static TREE_SNAPSHOT: LazyLock<TreeSnapshot> = LazyLock::new(|| {
    let new = TreeSnapshot::new();
    new.start_loop();
    new
});
