use arrayvec::ArrayString;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::executor;
use crate::public::tree::start_loop::SHOULD_RESET;
use crate::public::tree::TREE;
use crate::public::tree_snapshot::TREE_SNAPSHOT;
use rocket::serde::json::Json;
use serde::Deserialize;
#[derive(Debug, Deserialize)]
pub struct RegenerateData {
    #[serde(rename = "indexArray")]
    index_array: Vec<usize>,
    timestamp: String,
}
#[post("/put/regenerate-preview", format = "json", data = "<json_data>")]
pub async fn regenerate_preview(json_data: Json<RegenerateData>) -> () {
    tokio::task::spawn_blocking(move || {
        let table = TREE.read_tree_api();

        let reduced_data_vec = TREE_SNAPSHOT
            .read_tree_snapshot(&*json_data.timestamp)
            .unwrap();

        let hash_vec: Vec<ArrayString<64>> = json_data
            .index_array
            .iter()
            .map(|index| reduced_data_vec.get_hash(*index))
            .collect();
        let total_batches = (hash_vec.len() + 99) / 100;

        for (i, batch) in hash_vec.chunks(100).enumerate() {
            info!("Processing batch {}/{}", i + 1, total_batches);

            let iterator = batch.into_par_iter().map(|string| {
                let database = table.get(&**string).unwrap().unwrap().value();
                database
            });

            executor::compressor::compressor(iterator);
            SHOULD_RESET.notify_one();
        }
    })
    .await
    .unwrap()
}
