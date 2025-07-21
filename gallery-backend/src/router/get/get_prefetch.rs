use crate::public::db::query_snapshot::QUERY_SNAPSHOT;
use crate::public::db::tree::TREE;
use crate::public::db::tree::VERSION_COUNT_TIMESTAMP;
use crate::public::db::tree_snapshot::TREE_SNAPSHOT;
use crate::public::structure::album::ResolvedShare;
use crate::public::structure::database_struct::database_timestamp::DatabaseTimestamp;
use crate::public::structure::expression::Expression;
use crate::public::structure::reduced_data::ReducedData;
use crate::router::AppResult;
use crate::router::claims::claims_timestamp::ClaimsTimestamp;
use crate::router::fairing::guard_share::GuardShare;
use crate::tasks::COORDINATOR;
use crate::tasks::batcher::flush_query_snapshot::FlushQuerySnapshotTask;
use crate::tasks::batcher::flush_tree_snapshot::FlushTreeSnapshotTask;

use anyhow::Context;
use anyhow::{Result, anyhow};
use bitcode::{Decode, Encode};
use rayon::iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};
use std::hash::Hasher;
use std::hash::{DefaultHasher, Hash};
use std::sync::atomic::Ordering;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

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

#[derive(Debug, Clone, Deserialize, Serialize, Decode, Encode)]
#[serde(rename_all = "camelCase")]
pub struct PrefetchReturn {
    pub prefetch: Prefetch,
    pub token: String,
    pub resolved_share_opt: Option<ResolvedShare>,
}

impl PrefetchReturn {
    fn new(prefetch: Prefetch, token: String, resolved_share_opt: Option<ResolvedShare>) -> Self {
        Self {
            prefetch,
            token,
            resolved_share_opt,
        }
    }
}

// -----------------------------------------------------------------------------
// Convenience: &DatabaseTimestamp → ReducedData
// -----------------------------------------------------------------------------
impl From<&DatabaseTimestamp> for ReducedData {
    fn from(source: &DatabaseTimestamp) -> Self {
        Self {
            hash: source.abstract_data.hash(),
            width: source.abstract_data.width(),
            height: source.abstract_data.height(),
            date: source.timestamp,
        }
    }
}

// -----------------------------------------------------------------------------
// ── Single prefetch function ─────────────────────────────────────────────────
// -----------------------------------------------------------------------------

fn execute_prefetch_logic(
    expression_option: Option<Expression>,
    locate_option: Option<String>,
    resolved_share_option: Option<ResolvedShare>,
) -> Result<Json<PrefetchReturn>> {
    // Build cache key
    let mut hasher = DefaultHasher::new();
    expression_option.hash(&mut hasher);
    VERSION_COUNT_TIMESTAMP
        .load(Ordering::Relaxed)
        .hash(&mut hasher);
    locate_option.hash(&mut hasher);
    let query_hash = hasher.finish();

    // Check cache first
    if let Ok(Some(prefetch)) = QUERY_SNAPSHOT.read_query_snapshot(query_hash) {
        let claims = ClaimsTimestamp::new(resolved_share_option, prefetch.timestamp);
        return Ok(Json(PrefetchReturn::new(
            prefetch,
            claims.encode(),
            claims.resolved_share_opt,
        )));
    }

    // Collect data based on share or edit mode
    let tree_guard = TREE.in_memory.read().map_err(|err| anyhow!("{:?}", err))?;
    let reduced_data_vector: Vec<ReducedData> =
        match (expression_option.as_ref(), &resolved_share_option) {
            // If we have a resolved share then it must have a filter expression
            (Some(expr), Some(resolved_share)) => {
                let filter_fn = if resolved_share.share.show_metadata {
                    expr.clone().generate_filter()
                } else {
                    expr.clone()
                        .generate_filter_hide_metadata(resolved_share.album_id)
                };
                tree_guard
                    .par_iter()
                    .filter(|db_ts| filter_fn(&db_ts.abstract_data))
                    .map(|db_ts| db_ts.into())
                    .collect()
            }
            (Some(expr), None) => {
                let filter_fn = expr.clone().generate_filter();
                tree_guard
                    .par_iter()
                    .filter(|database_timestamp| filter_fn(&database_timestamp.abstract_data))
                    .map(|database_timestamp| database_timestamp.into())
                    .collect()
            }
            (None, _) => tree_guard
                .par_iter()
                .map(|database_timestamp| database_timestamp.into())
                .collect(),
        };

    // Find locate index if requested
    let locate_to_index = locate_option.as_ref().and_then(|hash| {
        reduced_data_vector
            .par_iter()
            .position_first(|reduced| reduced.hash.as_str() == hash)
    });

    // Persist to snapshot
    let timestamp_millis = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis();

    TREE_SNAPSHOT
        .in_memory
        .insert(timestamp_millis, reduced_data_vector);
    COORDINATOR.execute_batch_detached(FlushTreeSnapshotTask);

    let prefetch = Prefetch::new(
        timestamp_millis,
        locate_to_index,
        TREE_SNAPSHOT
            .in_memory
            .get(&timestamp_millis)
            .context(format!(
                "Failed to get prefetch data for timestamp {timestamp_millis}"
            ))?
            .len(),
    );

    // Cache the result
    QUERY_SNAPSHOT.in_memory.insert(query_hash, prefetch);
    COORDINATOR.execute_batch_detached(FlushQuerySnapshotTask);

    // Build response
    let claims = ClaimsTimestamp::new(resolved_share_option, timestamp_millis);
    Ok(Json(PrefetchReturn::new(
        prefetch,
        claims.encode(),
        claims.resolved_share_opt,
    )))
}

#[post("/get/prefetch?<locate>", format = "json", data = "<query_data>")]
pub async fn prefetch(
    auth_guard: GuardShare,
    query_data: Option<Json<Expression>>,
    locate: Option<String>,
) -> AppResult<Json<PrefetchReturn>> {
    // Combine album filter (if any) with the client‑supplied query.
    let mut combined_expression_option = query_data.map(|wrapper| wrapper.into_inner());
    let resolved_share_option = auth_guard.claims.get_share();

    if let Some(resolved_share) = &resolved_share_option {
        let album_filter_expression = Expression::Album(resolved_share.album_id);

        combined_expression_option = Some(match combined_expression_option {
            Some(client_expression) => {
                Expression::And(vec![album_filter_expression, client_expression])
            }
            None => album_filter_expression,
        });
    }

    // Execute on blocking thread
    let job_handle = tokio::task::spawn_blocking(move || {
        execute_prefetch_logic(combined_expression_option, locate, resolved_share_option)
    })
    .await??;

    Ok(job_handle)
}
