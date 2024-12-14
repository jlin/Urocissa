use crate::public::expression::Expression;
use crate::public::query_snapshot::start_loop::SHOULD_FLUSH_QUERY_SNAPSHOT;
use crate::public::query_snapshot::QUERY_SNAPSHOT;
use crate::public::reduced_data::ReducedData;
use crate::public::tree::start_loop::VERSION_COUNT_TIMESTAMP;
use crate::public::tree::TREE;
use crate::public::tree_snapshot::start_loop::SHOULD_FLUSH_TREE_SNAPSHOT;
use crate::public::tree_snapshot::TREE_SNAPSHOT;
use crate::public::utils::info_wrap;
use crate::router::fairing::AuthGuard;

use bitcode::{Decode, Encode};
use rayon::iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};
use std::hash::Hasher;
use std::hash::{DefaultHasher, Hash};
use std::sync::atomic::Ordering;
use std::time::UNIX_EPOCH;
use std::time::{Instant, SystemTime};

#[derive(Debug, Clone, Copy, Deserialize, Serialize, Decode, Encode)]
#[serde(rename_all = "camelCase")]
pub struct Prefetch {
    pub timestamp: u128,
    pub locate_to: Option<usize>,
    pub data_length: usize,
}

impl Prefetch {
    fn new(timestamp: u128, locate_to: Option<usize>, data_length: usize) -> Self {
        Self {
            timestamp,
            locate_to,
            data_length,
        }
    }
}
#[post("/get/prefetch?<locate>", format = "json", data = "<query_data>")]
pub async fn prefetch(
    _auth: AuthGuard,
    query_data: Option<Json<Expression>>,
    locate: Option<String>,
) -> Json<Option<Prefetch>> {
    tokio::task::spawn_blocking(move || {
        // Start timer
        let start_time = Instant::now();

        // Step 1: Check if query cache is available
        let find_cache_start_time = Instant::now();

        let expression_opt = query_data.map(|query| query.into_inner());

        let hasher = &mut DefaultHasher::new();

        expression_opt.hash(hasher);
        VERSION_COUNT_TIMESTAMP.load(Ordering::Relaxed).hash(hasher);

        let expression_hashed = hasher.finish();

        if let Ok(Some(prefetch_opt)) = QUERY_SNAPSHOT.read_query_snapshot(expression_hashed) {
            info_wrap(Some(find_cache_start_time.elapsed()), "Query cache found");
            return Json(prefetch_opt);
        } else {
            info_wrap(
                Some(find_cache_start_time.elapsed()),
                "Query cache not found. Generate a new one.",
            );
        }

        // Step 2: Filter items
        let filter_items_start_time = Instant::now();
        let ref_data = TREE.in_memory.read().unwrap();
        info_wrap(Some(filter_items_start_time.elapsed()), "Filter items");

        // Step 3: Compute layout
        let layout_start_time = Instant::now();

        let reduced_data: Vec<ReducedData> = match expression_opt {
            Some(expression) => {
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
        info_wrap(Some(layout_start_time.elapsed()), "Compute layout");

        // Step 4: Locate hash
        let locate_start_time = Instant::now();
        let locate_to = if let Some(ref locate_hash) = locate {
            reduced_data
                .par_iter()
                .position_first(|data| data.hash.as_str() == locate_hash)
        } else {
            None
        };
        info_wrap(Some(locate_start_time.elapsed()), "Locate data");

        // Step 5: Insert data into TREE_SNAPSHOT
        let db_start_time = Instant::now();
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();

        TREE_SNAPSHOT.in_memory.insert(timestamp, reduced_data);
        SHOULD_FLUSH_TREE_SNAPSHOT.notify_one();

        info_wrap(Some(db_start_time.elapsed()), "Write cache into memory");

        // Step 6: Create and return JSON response
        let json_start_time = Instant::now();

        let prefetch_opt = Some(Prefetch::new(timestamp, locate_to, data_length));

        QUERY_SNAPSHOT
            .in_memory
            .insert(expression_hashed, prefetch_opt);
        SHOULD_FLUSH_QUERY_SNAPSHOT.notify_one();
        let json = Json(prefetch_opt);

        info_wrap(Some(json_start_time.elapsed()), "Create JSON response");

        // Total elapsed time
        info_wrap(
            Some(start_time.elapsed()),
            "(total time) Get_data_length complete",
        );
        json
    })
    .await
    .unwrap()
}
