use super::TreeSnapshot;
use crate::public::tree::{read_tags::TagInfo, TREE};
use dashmap::DashMap;
use rayon::iter::{IntoParallelRefIterator, ParallelBridge, ParallelIterator};
use redb::ReadableTable;
use std::sync::atomic::{AtomicUsize, Ordering};

impl TreeSnapshot {
    pub fn read_tags(&'static self) -> Vec<TagInfo> {
        let tag_counts = DashMap::new();

        TREE.api_read_tree()
            .iter()
            .unwrap()
            .par_bridge()
            .for_each(|guard| {
                let (_, data) = guard.unwrap();
                for tag in data.value().tag {
                    let counter = tag_counts
                        .entry(tag.clone())
                        .or_insert_with(|| AtomicUsize::new(0));
                    counter.fetch_add(1, Ordering::Relaxed);
                }
            });

        let tag_infos: Vec<TagInfo> = tag_counts
            .par_iter()
            .map(|entry| TagInfo {
                tag: entry.key().clone(),
                number: entry.value().load(Ordering::Relaxed),
            })
            .collect();
        tag_infos
    }
}
