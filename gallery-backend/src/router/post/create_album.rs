use anyhow::anyhow;
use anyhow::{Context, Result};
use arrayvec::ArrayString;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use rocket::post;
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};
use std::time::Instant;

use crate::operations::hash::generate_random_hash;
use crate::operations::open_db::open_data_table;
use crate::process::transitor::index_to_database;

use crate::public::structure::abstract_data::AbstractData;
use crate::tasks::actor::album::AlbumTask;

use crate::public::db::tree_snapshot::TREE_SNAPSHOT;
use crate::public::structure::album::Album;
use crate::router::AppResult;
use crate::router::fairing::guard_auth::GuardAuth;
use crate::router::fairing::guard_read_only_mode::GuardReadOnlyMode;
use crate::tasks::COORDINATOR;
use crate::tasks::batcher::flush_tree::FlushTreeTask;
use crate::tasks::batcher::update_tree::UpdateTreeTask;

#[derive(Debug, Clone, Deserialize, Default, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CreateAlbum {
    pub title: Option<String>,
    pub elements_index: Vec<usize>,
    pub timestamp: u128,
}

async fn create_album_internal(
    title: Option<String>,
    elements_index: Vec<usize>,
    timestamp: Option<u128>,
) -> Result<ArrayString<64>> {
    let start_time = Instant::now();

    let album_id = generate_random_hash();
    let album = AbstractData::Album(Album::new(album_id, title));
    COORDINATOR
        .execute_batch_waiting(FlushTreeTask::insert(vec![album]))
        .await?;

    if !elements_index.is_empty() {
        let mut batch = tokio::task::spawn_blocking(move || -> Result<Vec<AbstractData>> {
            let ts = timestamp.context("timestamp required for non‑empty album")?;

            // expensive *sync* look‑ups can stay in Rayon
            let tree_snapshot = TREE_SNAPSHOT
                .read_tree_snapshot(&ts)
                .context("failed to read tree snapshot")?;
            let data_table = open_data_table();

            let batch: Vec<_> = elements_index
                .into_par_iter()
                .map(|idx| {
                    let mut db = index_to_database(&tree_snapshot, &data_table, idx)
                        .map_err(|e| anyhow!("convert index {idx}: {e}"))?;
                    db.album.insert(album_id);
                    Ok(AbstractData::Database(db))
                })
                .collect::<Result<_>>()?;
            Ok(batch)
        })
        .await??;
        // single async flush – safe to await here
        COORDINATOR
            .execute_batch_waiting(FlushTreeTask::insert(std::mem::take(&mut batch)))
            .await?;
    }

    // --- 3. post‑creation maintenance tasks -------------------------------
    COORDINATOR.execute_batch_waiting(UpdateTreeTask).await?;
    COORDINATOR
        .execute_waiting(AlbumTask::new(album_id))
        .await??;

    info!(duration = &*format!("{:?}", start_time.elapsed()); "Create album");
    Ok(album_id)
}

#[post("/post/create_non_empty_album", data = "<create_album>")]
pub async fn create_non_empty_album(
    _auth: GuardAuth,
    _read_only_mode: GuardReadOnlyMode,
    create_album: Json<CreateAlbum>,
) -> AppResult<String> {
    let create_album = create_album.into_inner();
    let album_id = create_album_internal(
        create_album.title,
        create_album.elements_index,
        Some(create_album.timestamp),
    )
    .await?;

    Ok(album_id.to_string())
}

#[post("/post/create_empty_album")]
pub async fn create_empty_album(
    _auth: GuardAuth,
    _read_only_mode: GuardReadOnlyMode,
) -> AppResult<String> {
    let album_id = create_album_internal(None, Vec::new(), None).await?;
    Ok(album_id.to_string())
}
