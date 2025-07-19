use crate::public::db::tree_snapshot::TREE_SNAPSHOT;
use anyhow::Result;
use anyhow::anyhow;
use arrayvec::ArrayString;
use rocket::http::private::Array;

pub fn read_hash_by_index(
    timestamp: u128,
    index_list: Vec<usize>,
) -> impl Iterator<Item = ArrayString<64>> {
    let tree_snapshot = TREE_SNAPSHOT.read_tree_snapshot(&timestamp).unwrap();
    index_list
        .into_iter()
        .map(move |index| tree_snapshot.get_hash(index))
}
