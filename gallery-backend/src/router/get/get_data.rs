use crate::public::config::{PrivateConfig, PRIVATE_CONFIG};
use crate::public::database_struct::database_timestamp::DataBaseTimestamp;
use crate::public::expression::Expression;
use crate::public::redb::DATA_TABLE;
use crate::public::row::{Row, ScrollBarData};
use crate::public::tree::read_tags::TagInfo;
use crate::public::tree::TREE;
use crate::public::tree_snap_shot_in_memory::ReducedData;
use crate::public::tree_snapshot::TREE_SNAPSHOT;
use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};

use rocket::http::Status;
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};
use std::time::UNIX_EPOCH;
use std::time::{Instant, SystemTime};
#[post("/get/get-db-length?<locate>", format = "json", data = "<query_data>")]
pub async fn get_data_length(
    query_data: Option<Json<Expression>>,
    locate: Option<String>,
) -> Json<Option<Prefetch>> {
    tokio::task::spawn_blocking(move || {
        // Start timer
        let start_time = Instant::now();

        // Step 1: Generate filter from expression
        let elapsed = start_time.elapsed().as_millis();
        println!("Step 1 (Generate filter): {} ms", elapsed);

        // Step 2: Filter items
        let filter_items_start_time = Instant::now();
        let ref_data = TREE.in_memory.read().unwrap();
        let filter_items_elapsed = filter_items_start_time.elapsed().as_millis();
        println!("Step 2 (Filter items): {} ms", filter_items_elapsed);

        // Step 3: Compute layout
        let layout_start_time = Instant::now();
        let reduced_data: Vec<ReducedData> = match query_data {
            Some(query) => {
                let expression = query.into_inner();
                let filter = expression.generate_filter();
                ref_data
                    .par_iter()
                    .filter(move |database| filter(&database.database))
                    .map(|item| ReducedData {
                        hash: item.database.hash,
                        width: item.database.width,
                        height: item.database.height,
                        date: item.timestamp,
                    })
                    .collect()
            }
            None => ref_data
                .par_iter()
                .map(|item| ReducedData {
                    hash: item.database.hash,
                    width: item.database.width,
                    height: item.database.height,
                    date: item.timestamp,
                })
                .collect(),
        };

        let data_length = reduced_data.len();
        let layout_elapsed = layout_start_time.elapsed().as_millis();
        println!("Step 3 (Compute layout): {} ms", layout_elapsed);

        // Step 4: Locate hash
        let locate_start_time = Instant::now();
        let locate_to = if let Some(ref locate_hash) = locate {
            reduced_data
                .iter()
                .position(|data| data.hash.as_str() == locate_hash)
        } else {
            None
        };
        let locate_elapsed = locate_start_time.elapsed().as_millis();
        println!("Step 4 (Locate hash): {} ms", locate_elapsed);

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
        let db_elapsed = db_start_time.elapsed().as_millis();
        println!("Step 5 (Insert data into TREE_SNAPSHOT): {} ms", db_elapsed);

        // Step 6: Create and return JSON response
        let json_start_time = Instant::now();
        let json = Json(Some(Prefetch::new(
            timestamp.clone(),
            locate_to,
            data_length,
        )));
        let json_elapsed = json_start_time.elapsed().as_millis();
        println!(
            "Step 6 (Create and return JSON response): {} ms",
            json_elapsed
        );

        // Total elapsed time
        let total_elapsed = start_time.elapsed().as_millis();
        println!("Total elapsed time: {} ms", total_elapsed);

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
        let end = end.min(tree_snapshot.len());
        if start < end {
            let data_vec: Vec<DataBaseTimestamp> = (start..end)
                .into_par_iter()
                .map(|index| {
                    let database = table
                        .get(&*tree_snapshot.get_hash(index))
                        .unwrap()
                        .unwrap()
                        .value();
                    DataBaseTimestamp::new(
                        database,
                        &vec!["DateTimeOriginal", "filename", "modified", "scan_time"],
                    )
                })
                .collect();
            let read_time = start_time.elapsed();
            println!("get_data: {:?}", read_time);
            Ok(Json(data_vec))
        } else {
            // index out of range
            Ok(Json(vec![]))
        }
    })
    .await
    .unwrap()
}

#[get("/get/get-config.json")]
pub async fn get_config() -> Json<&'static PrivateConfig> {
    Json(&*PRIVATE_CONFIG)
}

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

#[get("/get/get-tags")]
pub async fn get_tags() -> Json<Vec<TagInfo>> {
    tokio::task::spawn_blocking(move || {
        let vec_tags_info = TREE.read_tags();
        Json(vec_tags_info)
    })
    .await
    .unwrap()
}

#[get("/get/get-rows?<index>&<timestamp>")]
pub async fn get_rows(index: usize, timestamp: String) -> Result<Json<Row>, Status> {
    tokio::task::spawn_blocking(move || {
        let start_time = Instant::now();
        let filtered_rows = TREE_SNAPSHOT.read_row(index, timestamp)?;
        let duration = start_time.elapsed();
        println!(
            "Time taken to read rows and create RowWithWidth: {:?}",
            duration
        );
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
