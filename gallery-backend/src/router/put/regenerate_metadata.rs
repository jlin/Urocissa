use arrayvec::ArrayString;
use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};
use rocket::http::Status;

use crate::executor::databaser::processor::{
    regenerate_metadata_for_image, regenerate_metadata_for_video,
};
use crate::public::constant::PROCESS_BATCH_NUMBER;
use crate::public::tree::start_loop::SHOULD_RESET;
use crate::public::tree::TREE;
use crate::public::tree_snapshot::TREE_SNAPSHOT;
use crate::router::fairing::{AuthGuard, ReadOnlyModeGuard};
use rocket::serde::json::Json;
use serde::Deserialize;
#[derive(Debug, Deserialize)]
pub struct RegenerateData {
    #[serde(rename = "indexArray")]
    index_array: Vec<usize>,
    timestamp: u128,
}

#[post("/put/regenerate-metadata", format = "json", data = "<json_data>")]
pub async fn regenerate_metadata(
    _auth: AuthGuard,
    _read_only_mode: ReadOnlyModeGuard,
    json_data: Json<RegenerateData>,
) -> Status {
    let json_data = json_data.into_inner();
    tokio::task::spawn_blocking(move || {
        let table = TREE.read_tree_api();

        let reduced_data_vec = TREE_SNAPSHOT
            .read_tree_snapshot(&json_data.timestamp)
            .unwrap();
        let hash_vec: Vec<ArrayString<64>> = json_data
            .index_array
            .par_iter()
            .map(|index| reduced_data_vec.get_hash(*index))
            .collect();
        let total_batches = (hash_vec.len() + PROCESS_BATCH_NUMBER - 1) / PROCESS_BATCH_NUMBER;

        for (i, batch) in hash_vec.chunks(PROCESS_BATCH_NUMBER).enumerate() {
            info!("Processing batch {}/{}", i + 1, total_batches);

            let list_of_database: Vec<_> = batch
                .into_par_iter()
                .filter_map(|string| {
                    let mut database = table.get(&**string).unwrap().unwrap().value();
                    if database.ext_type == "image" {
                        match regenerate_metadata_for_image(&mut database) {
                            Ok(_) => Some(database),
                            Err(_) => None,
                        }
                    } else if database.ext_type == "video" {
                        match regenerate_metadata_for_video(&mut database) {
                            Ok(_) => Some(database),
                            Err(_) => None,
                        }
                    } else {
                        None
                    }
                })
                .collect();
            TREE.insert_tree_api(&list_of_database).unwrap();
        }
    })
    .await
    .unwrap();
    TREE.should_update().await;
    Status::Ok
}
