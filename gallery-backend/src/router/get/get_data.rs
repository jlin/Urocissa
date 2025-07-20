use crate::operations::resolve_show_download_and_metadata;
use crate::process::get_data::get_data_process;
use crate::public::db::tree_snapshot::TREE_SNAPSHOT;
use crate::public::structure::database_struct::database_timestamp::DataBaseTimestampReturn;
use crate::public::structure::row::{Row, ScrollBarData};

use crate::router::AppResult;
use crate::router::fairing::guard_timestamp::GuardTimestamp;
use log::info;

use rocket::serde::json::Json;
use std::time::Instant;

#[get("/get/get-data?<timestamp>&<start>&<end>")]
pub async fn get_data(
    guard_timestamp: GuardTimestamp,
    timestamp: u128,
    start: usize,
    end: usize,
) -> AppResult<Json<Vec<DataBaseTimestampReturn>>> {
    tokio::task::spawn_blocking(move || {
        let start_time = Instant::now();

        let resolved_share_opt = guard_timestamp.claims.resolved_share_opt;
        let (show_download, show_metadata) = resolve_show_download_and_metadata(resolved_share_opt);

        let database_timestamp_return_list =
            get_data_process(timestamp, start, end, show_download, show_metadata)?;

        let duration = format!("{:?}", start_time.elapsed());
        info!(duration = &*duration; "Get data: {} ~ {}", start, end);
        Ok(Json(database_timestamp_return_list))
    })
    .await
    .unwrap()
}

#[get("/get/get-rows?<index>&<timestamp>")]
pub async fn get_rows(
    _auth: GuardTimestamp,
    index: usize,
    timestamp: u128,
) -> AppResult<Json<Row>> {
    tokio::task::spawn_blocking(move || {
        let start_time = Instant::now();
        let filtered_rows = TREE_SNAPSHOT.read_row(index, timestamp)?;
        let duration = format!("{:?}", start_time.elapsed());
        info!(duration = &*duration; "Read rows: index = {}", index);
        return Ok(Json(filtered_rows));
    })
    .await
    .unwrap()
}

#[get("/get/get-scroll-bar?<timestamp>")]
pub async fn get_scroll_bar(_auth: GuardTimestamp, timestamp: u128) -> Json<Vec<ScrollBarData>> {
    let scrollbar_data = TREE_SNAPSHOT.read_scrollbar(timestamp);
    Json(scrollbar_data)
}
