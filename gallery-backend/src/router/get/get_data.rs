use crate::public::abstract_data::AbstractData;
use crate::public::album::Album;
use crate::public::config::{PublicConfig, PUBLIC_CONFIG};
use crate::public::database_struct::database_timestamp::DataBaseTimestamp;
use crate::public::expression::Expression;
use crate::public::redb::{ALBUM_TABLE, DATA_TABLE};
use crate::public::reduced_data::ReducedData;
use crate::public::row::{Row, ScrollBarData};
use crate::public::tree::read_tags::TagInfo;
use crate::public::tree::TREE;
use crate::public::tree_snapshot::TREE_SNAPSHOT;
use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};

use rocket::http::Status;
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};
use std::time::UNIX_EPOCH;
use std::time::{Instant, SystemTime};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Prefetch {
    pub timestamp: String,
    pub locate_to: Option<usize>,
    pub data_length: usize,
}

impl Prefetch {
    fn new(timestamp: String, locate_to: Option<usize>, data_length: usize) -> Self {
        Self {
            timestamp,
            locate_to,
            data_length,
        }
    }
}

#[post("/get/prefetch?<locate>", format = "json", data = "<query_data>")]
pub async fn prefetch(
    query_data: Option<Json<Expression>>,
    locate: Option<String>,
) -> Json<Option<Prefetch>> {
    tokio::task::spawn_blocking(move || {
        // Start timer
        let start_time = Instant::now();

        // Step 1: Generate filter from expression
        info!(duration = &*format!("{:?}", start_time.elapsed()); "Generate filter");

        // Step 2: Filter items
        let filter_items_start_time = Instant::now();
        let ref_data = TREE.in_memory.read().unwrap();
        info!(duration = &*format!("{:?}", filter_items_start_time.elapsed()); "Filter items");

        // Step 3: Compute layout
        let layout_start_time = Instant::now();
        let reduced_data: Vec<ReducedData> = match query_data {
            Some(query) => {
                let expression = query.into_inner();
                let filter = expression.generate_filter();
                ref_data
                    .par_iter()
                    .filter(move |database_timestamp| filter(&database_timestamp.abstract_data))
                    .map(|database_timestamp| ReducedData {
                        hash: database_timestamp.abstract_data.hash(),
                        width: database_timestamp.abstract_data.width(),
                        height: database_timestamp.abstract_data.height(),
                        date: database_timestamp.timestamp,
                    })
                    .collect()
            }
            None => ref_data
                .par_iter()
                .map(|database_timestamp| ReducedData {
                    hash: database_timestamp.abstract_data.hash(),
                    width: database_timestamp.abstract_data.width(),
                    height: database_timestamp.abstract_data.height(),
                    date: database_timestamp.timestamp,
                })
                .collect(),
        };

        let data_length = reduced_data.len();
        info!(duration = &*format!("{:?}", layout_start_time.elapsed()); "Compute layout");

        // Step 4: Locate hash
        let locate_start_time = Instant::now();
        let locate_to = if let Some(ref locate_hash) = locate {
            reduced_data
                .iter()
                .position(|data| data.hash.as_str() == locate_hash)
        } else {
            None
        };
        info!(duration = &*format!("{:?}", locate_start_time.elapsed()); "Locate data");

        // Step 5: Insert data into TREE_SNAPSHOT
        let db_start_time = Instant::now();
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis()
            .to_string();
        TREE_SNAPSHOT
            .in_memory
            .insert(timestamp.clone(), reduced_data);

        info!(duration = &*format!("{:?}", db_start_time.elapsed()); "Write cache into memory");

        // Step 6: Create and return JSON response
        let json_start_time = Instant::now();
        let json = Json(Some(Prefetch::new(
            timestamp.clone(),
            locate_to,
            data_length,
        )));
        info!(duration = &*format!("{:?}", json_start_time.elapsed()); "Create JSON response");

        // Total elapsed time
        info!(duration = &*format!("{:?}", start_time.elapsed()); "get_data_length complete");
        json
    })
    .await
    .unwrap()
}

#[get("/get/get-data?<timestamp>&<start>&<end>")]
pub async fn get_data(
    timestamp: String,
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
                    warn!(duration = &*format!("{:?}", start_time.elapsed()); "Get data: {} ~ {}", start, end);
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
pub async fn get_config() -> Json<&'static PublicConfig> {
    Json(&*PUBLIC_CONFIG)
}

#[get("/get/get-tags")]
pub async fn get_tags() -> Json<Vec<TagInfo>> {
    tokio::task::spawn_blocking(move || {
        let vec_tags_info = TREE.read_tags();
        Json(vec_tags_info)
    })
    .await
    .unwrap()
}

#[get("/get/get-albums")]
pub async fn get_albums() -> Json<Vec<Album>> {
    tokio::task::spawn_blocking(move || {
        let album_list = TREE.read_albums();
        Json(album_list)
    })
    .await
    .unwrap()
}

#[get("/get/get-rows?<index>&<timestamp>")]
pub async fn get_rows(index: usize, timestamp: String) -> Result<Json<Row>, Status> {
    tokio::task::spawn_blocking(move || {
        let start_time = Instant::now();
        let filtered_rows = TREE_SNAPSHOT.read_row(index, timestamp)?;
        info!(duration = &*format!("{:?}", start_time.elapsed()); "Read rows: index = {}", index);
        return Ok(Json(filtered_rows));
    })
    .await
    .unwrap()
}

#[get("/get/get-scroll-bar?<timestamp>")]
pub async fn get_scroll_bar(timestamp: String) -> Result<Json<Vec<ScrollBarData>>, Status> {
    let scrollbar_data = TREE_SNAPSHOT.read_scrollbar(timestamp);
    Ok(Json(scrollbar_data))
}
