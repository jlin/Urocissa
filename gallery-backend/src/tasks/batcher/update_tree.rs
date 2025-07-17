use crate::public::db::expire::EXPIRE;
use crate::public::db::tree::TREE;
use crate::public::structure::abstract_data::AbstractData;
use crate::public::structure::database_struct::database_timestamp::DatabaseTimestamp;
use rayon::iter::{ParallelBridge, ParallelIterator};
use rayon::prelude::ParallelSliceMut;
use redb::ReadableTable;
use std::collections::HashSet;
use std::sync::LazyLock;
use std::time::Instant;

static ALLOWED_KEYS: LazyLock<HashSet<&'static str>> = LazyLock::new(|| {
    [
        "Make",
        "Model",
        "FNumber",
        "ExposureTime",
        "FocalLength",
        "PhotographicSensitivity",
        "DateTimeOriginal",
        "duration",
        "rotation",
    ]
    .iter()
    .cloned()
    .collect()
});

pub struct UpdateTreeTask;

impl mini_coordinator::BatchTask for UpdateTreeTask {
    fn batch_run(_: Vec<Self>) -> impl std::future::Future<Output = ()> + Send {
        async move {
            update_tree_task();
        }
    }
}

pub fn update_tree_task() {
    let start_time = Instant::now();
    let table = TREE.api_read_tree();

    let priority_list = vec!["DateTimeOriginal", "filename", "modified", "scan_time"];

    let mut data_vec: Vec<DatabaseTimestamp> = table
        .iter()
        .unwrap()
        .par_bridge()
        .map(|guard| {
            let (_, value) = guard.unwrap();
            let mut database = value.value();
            // retain only necessary exif data used for query search
            database
                .exif_vec
                .retain(|k, _| ALLOWED_KEYS.contains(&k.as_str()));
            DatabaseTimestamp::new(AbstractData::Database(database), &priority_list)
        })
        .collect();

    let album_table = TREE.api_read_album();

    let album_vec: Vec<DatabaseTimestamp> = album_table
        .iter()
        .unwrap()
        .par_bridge()
        .map(|guard| {
            let (_, value) = guard.unwrap();
            let album = value.value();
            DatabaseTimestamp::new(AbstractData::Album(album), &priority_list)
        })
        .collect();

    data_vec.extend(album_vec);
    data_vec.par_sort_by(|a, b| b.timestamp.cmp(&a.timestamp));

    *TREE.in_memory.write().unwrap() = data_vec;

    EXPIRE.update_expire_time(start_time);
}
