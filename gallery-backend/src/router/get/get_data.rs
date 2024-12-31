use crate::public::abstract_data::AbstractData;
use crate::public::config::{PublicConfig, PUBLIC_CONFIG};
use crate::public::database_struct::database_timestamp::DataBaseTimestamp;
use crate::public::redb::{ALBUM_TABLE, DATA_TABLE};
use crate::public::row::{Row, ScrollBarData};
use crate::public::tree::read_tags::TagInfo;
use crate::public::tree::TREE;
use crate::public::tree_snapshot::TREE_SNAPSHOT;
use crate::public::utils::{info_wrap, warn_wrap};
use crate::router::fairing::AuthGuard;

use rayon::iter::{IntoParallelIterator, ParallelIterator};
use rocket::http::Status;
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};
use std::hash::Hash;
use std::time::Instant;

#[get("/get/get-data?<timestamp>&<start>&<end>")]
pub async fn get_data(
    _auth: AuthGuard,
    timestamp: u128,
    start: usize,
    end: usize,
) -> Result<Json<Vec<DataBaseTimestamp>>, Status> {
    tokio::task::spawn_blocking(move || {
        let start_time = Instant::now();
        let tree_snapshot = TREE_SNAPSHOT.read_tree_snapshot(&timestamp).unwrap();
        let read_txn = TREE.in_disk.begin_read().unwrap();
        let table = read_txn.open_table(DATA_TABLE).unwrap();
        let album_table = read_txn.open_table(ALBUM_TABLE).unwrap();
        let end = end.min(tree_snapshot.len());

        if start < end {
            // Change the type of data_vec to Result<Vec<DataBaseTimestamp>, Status>
            let data_vec: Result<Vec<DataBaseTimestamp>, Status> = (start..end)
                .into_par_iter()
                .map(
                    |index| match table.get(&*tree_snapshot.get_hash(index)).unwrap() {
                        Some(database) => Ok(DataBaseTimestamp::new(
                            AbstractData::DataBase(database.value()),
                            &vec!["DateTimeOriginal", "filename", "modified", "scan_time"],
                        )),
                        None => match album_table.get(&*tree_snapshot.get_hash(index)).unwrap() {
                            Some(album) => Ok(DataBaseTimestamp::new(
                                AbstractData::Album(album.value()),
                                &vec!["DateTimeOriginal", "filename", "modified", "scan_time"],
                            )),
                            None => Err(Status::InternalServerError),
                        },
                    },
                )
                .collect();

            match data_vec {
                Ok(vec) => {
                    warn_wrap(
                        Some(start_time.elapsed()),
                        &format!("Get data: {} ~ {}", start, end),
                    );
                    Ok(Json(vec))
                }
                Err(e) => Err(e),
            }
        } else {
            // Index out of range
            Ok(Json(vec![]))
        }
    })
    .await
    .unwrap()
}

#[get("/get/get-config.json")]
pub async fn get_config(_auth: AuthGuard) -> Json<&'static PublicConfig> {
    Json(&*PUBLIC_CONFIG)
}

#[get("/get/get-tags")]
pub async fn get_tags(_auth: AuthGuard) -> Json<Vec<TagInfo>> {
    tokio::task::spawn_blocking(move || {
        let vec_tags_info = TREE.read_tags();
        Json(vec_tags_info)
    })
    .await
    .unwrap()
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct AlbumInfo {
    pub album_id: String,
    pub album_name: Option<String>,
}

#[get("/get/get-albums")]
pub async fn get_albums(_auth: AuthGuard) -> Json<Vec<AlbumInfo>> {
    tokio::task::spawn_blocking(move || {
        let album_list = TREE.read_albums();
        let album_info_list = album_list
            .into_iter()
            .map(|album| AlbumInfo {
                album_id: album.id.to_string(),
                album_name: album.title,
            })
            .collect();
        Json(album_info_list)
    })
    .await
    .unwrap()
}

#[get("/get/get-rows?<index>&<timestamp>")]
pub async fn get_rows(
    _auth: AuthGuard,
    index: usize,
    timestamp: u128,
) -> Result<Json<Row>, Status> {
    tokio::task::spawn_blocking(move || {
        let start_time = Instant::now();
        let filtered_rows = TREE_SNAPSHOT.read_row(index, timestamp)?;
        info_wrap(
            Some(start_time.elapsed()),
            &format!("Read rows: index = {}", index),
        );
        return Ok(Json(filtered_rows));
    })
    .await
    .unwrap()
}

#[get("/get/get-scroll-bar?<timestamp>")]
pub async fn get_scroll_bar(
    _auth: AuthGuard,
    timestamp: u128,
) -> Result<Json<Vec<ScrollBarData>>, Status> {
    let scrollbar_data = TREE_SNAPSHOT.read_scrollbar(timestamp);
    Ok(Json(scrollbar_data))
}
