use crate::public::db::query_snapshot::QUERY_SNAPSHOT;
use crate::public::db::tree::TREE;
use crate::public::db::tree::VERSION_COUNT_TIMESTAMP;
use crate::public::db::tree_snapshot::TREE_SNAPSHOT;
use crate::public::structure::album::ResolvedShare;
use crate::public::structure::database_struct::database_timestamp::DatabaseTimestamp;
use crate::public::structure::expression::Expression;
use crate::public::structure::reduced_data::ReducedData;
use crate::router::claims::claims_timestamp::ClaimsTimestamp;
use crate::router::fairing::guard_share::GuardShare;
use crate::tasks::COORDINATOR;
use crate::tasks::batcher::flush_query_snapshot::FLUSH_QUERY_SNAPSHOT_QUEUE;
use crate::tasks::batcher::flush_tree_snapshot::FlushTreeSnapshotTask;

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
// ── 1. Pure helpers (logic only, no side‑effects) ─────────────────────────────
// -----------------------------------------------------------------------------

/// Build a stable cache key from the expression, version counter and locate hash.
fn build_query_hash(expression_option: &Option<Expression>, locate_option: &Option<String>) -> u64 {
    let mut hasher = DefaultHasher::new();
    expression_option.hash(&mut hasher);
    VERSION_COUNT_TIMESTAMP
        .load(Ordering::Relaxed)
        .hash(&mut hasher);
    locate_option.hash(&mut hasher);
    hasher.finish()
}

/// Produce a vector of [`ReducedData`] that matches the expression (if any).
fn collect_reduced_data_edit(expression_option: Option<Expression>) -> Vec<ReducedData> {
    let tree_guard = TREE.in_memory.read().unwrap();

    match expression_option {
        Some(expression) => {
            let filter_fn = expression.generate_filter();
            tree_guard
                .par_iter()
                .filter(|database_timestamp| filter_fn(&database_timestamp.abstract_data))
                .map(|database_timestamp| database_timestamp.into())
                .collect()
        }
        None => tree_guard
            .par_iter()
            .map(|database_timestamp| database_timestamp.into())
            .collect(),
    }
}

fn collect_reduced_data_share(
    expression_option: Option<Expression>,
    resolved_share: &ResolvedShare,
) -> Vec<ReducedData> {
    let tree_guard = TREE.in_memory.read().unwrap();

    match expression_option {
        Some(expr) => {
            let filter_fn = if resolved_share.share.show_metadata {
                expr.generate_filter()
            } else {
                expr.generate_filter_hide_metadata(resolved_share.album_id)
            };

            tree_guard
                .par_iter()
                .filter(|db_ts| filter_fn(&db_ts.abstract_data))
                .map(|db_ts| db_ts.into())
                .collect()
        }
        None => tree_guard.par_iter().map(|db_ts| db_ts.into()).collect(),
    }
}

/// Locate the index for the requested hash, if the client supplied one.
fn locate_index(
    reduced_data_slice: &[ReducedData],
    locate_option: &Option<String>,
) -> Option<usize> {
    locate_option.as_ref().and_then(|hash| {
        reduced_data_slice
            .par_iter()
            .position_first(|reduced| reduced.hash.as_str() == hash)
    })
}

// -----------------------------------------------------------------------------
// ── 2. Helpers with side‑effects (snapshot, cache, JWT) ───────────────────────
// -----------------------------------------------------------------------------

/// Persist `reduced_data_vector` into `TREE_SNAPSHOT`; return `(timestamp, Prefetch)`.
fn persist_tree_snapshot(
    reduced_data_vector: Vec<ReducedData>,
    locate_to_index: Option<usize>,
) -> (u128, Prefetch) {
    let timestamp_millis = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();

    TREE_SNAPSHOT
        .in_memory
        .insert(timestamp_millis, reduced_data_vector);
    COORDINATOR.execute_batch_detached(FlushTreeSnapshotTask);

    (
        timestamp_millis,
        Prefetch::new(
            timestamp_millis,
            locate_to_index,
            TREE_SNAPSHOT
                .in_memory
                .get(&timestamp_millis)
                .unwrap()
                .len(),
        ),
    )
}

/// Insert `prefetch` into the query‑level cache.
fn cache_prefetch(query_hash: u64, prefetch: Prefetch) {
    QUERY_SNAPSHOT.in_memory.insert(query_hash, prefetch);
    FLUSH_QUERY_SNAPSHOT_QUEUE.update(vec![()]);
}

/// Assemble the JSON response for the **edit** path.
fn build_edit_response(prefetch: Prefetch, timestamp_millis: u128) -> Json<PrefetchReturn> {
    let claims = ClaimsTimestamp::new(None, timestamp_millis);
    Json(PrefetchReturn::new(prefetch, claims.encode(), None))
}

/// Assemble the JSON response for the **share** path.
fn build_share_response(
    prefetch: Prefetch,
    timestamp_millis: u128,
    resolved_share: ResolvedShare,
) -> Json<PrefetchReturn> {
    let claims = ClaimsTimestamp::new(Some(resolved_share), timestamp_millis);
    Json(PrefetchReturn::new(
        prefetch,
        claims.encode(),
        claims.resolved_share_opt,
    ))
}

// -----------------------------------------------------------------------------
// ── 3. Business helpers (edit • share) ────────────────────────────────────────
// -----------------------------------------------------------------------------

fn execute_edit_path(
    expression_option: Option<Expression>,
    locate_option: Option<String>,
) -> Json<PrefetchReturn> {
    let query_hash = build_query_hash(&expression_option, &locate_option);

    // A. cache hit?
    if let Ok(Some(prefetch)) = QUERY_SNAPSHOT.read_query_snapshot(query_hash) {
        return build_edit_response(prefetch, prefetch.timestamp);
    }

    // B. fresh computation
    let reduced_data_vector = collect_reduced_data_edit(expression_option);
    let locate_to_index = locate_index(&reduced_data_vector, &locate_option);
    let (timestamp_millis, prefetch) = persist_tree_snapshot(reduced_data_vector, locate_to_index);

    cache_prefetch(query_hash, prefetch);
    build_edit_response(prefetch, timestamp_millis)
}

fn execute_share_path(
    expression_option: Option<Expression>,
    locate_option: Option<String>,
    resolved_share: ResolvedShare,
) -> Json<PrefetchReturn> {
    let query_hash = build_query_hash(&expression_option, &locate_option);

    if let Ok(Some(prefetch)) = QUERY_SNAPSHOT.read_query_snapshot(query_hash) {
        return build_share_response(prefetch, prefetch.timestamp, resolved_share);
    }

    let reduced_data_vector = collect_reduced_data_share(expression_option, &resolved_share);
    let locate_to_index = locate_index(&reduced_data_vector, &locate_option);
    let (timestamp_millis, prefetch) = persist_tree_snapshot(reduced_data_vector, locate_to_index);

    cache_prefetch(query_hash, prefetch);
    build_share_response(prefetch, timestamp_millis, resolved_share)
}

// -----------------------------------------------------------------------------
// ── 4. Single merged Rocket route ─────────────────────────────────────────────
// -----------------------------------------------------------------------------

#[post("/get/prefetch?<locate>", format = "json", data = "<query_data>")]
pub async fn prefetch(
    auth_guard: GuardShare,
    query_data: Option<Json<Expression>>,
    locate: Option<String>,
) -> Json<PrefetchReturn> {
    // Combine album filter (if any) with the client‑supplied query.
    let mut combined_expression_option = query_data.map(|wrapper| wrapper.into_inner());

    let job_handle = if let Some(resolved_share) = auth_guard.claims.get_share() {
        let album_filter_expression = Expression::Album(resolved_share.album_id);

        combined_expression_option = Some(match combined_expression_option {
            Some(client_expression) => {
                Expression::And(vec![album_filter_expression, client_expression])
            }
            None => album_filter_expression,
        });

        // heavy work on blocking thread – share path
        tokio::task::spawn_blocking(move || {
            execute_share_path(combined_expression_option, locate, resolved_share)
        })
    } else {
        // edit path
        tokio::task::spawn_blocking(move || execute_edit_path(combined_expression_option, locate))
    };

    job_handle.await.unwrap()
}
