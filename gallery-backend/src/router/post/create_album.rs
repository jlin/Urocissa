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
use crate::process::transitor::index_to_abstract_database;

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

/// Creates an album with optional title and elements
async fn create_album_internal(
    title: Option<String>,
    elements_index: Vec<usize>,
    timestamp: Option<u128>,
) -> Result<ArrayString<64>> {
    let album_id = tokio::task::spawn_blocking(move || -> Result<ArrayString<64>> {
        let start_time = Instant::now();
        
        let album_id = generate_random_hash();
        let album_database = Album::new(album_id, title);
        let abstract_data_album = AbstractData::Album(album_database);
        
        // Insert the album first
        COORDINATOR.execute_batch_detached(FlushTreeTask::insert(vec![abstract_data_album]));
        
        // Add elements if provided
        if !elements_index.is_empty() {
            let timestamp = timestamp.context("Timestamp required for non-empty album")?;
            let tree_snapshot = TREE_SNAPSHOT
                .read_tree_snapshot(&timestamp)
                .context("Failed to read tree snapshot")?;
            let data_table = open_data_table();
            
            elements_index
                .into_par_iter()
                .try_for_each(|index| -> Result<()> {
                    let abstract_data = index_to_abstract_database(&tree_snapshot, &data_table, index)
                        .map_err(|e| anyhow!("Failed to convert index to abstract data: {}", e))?;

                    COORDINATOR.execute_batch_detached(FlushTreeTask::insert(vec![abstract_data]));
                    Ok(())
                })?;
        }

        info!(duration = &*format!("{:?}", start_time.elapsed()); "Create album");
        Ok(album_id)
    })
    .await
    .context("Failed to execute blocking task")?
    .context("Task execution failed")?;

    // Execute post-creation tasks
    COORDINATOR
        .execute_batch_waiting(UpdateTreeTask)
        .await
        .context("Failed to execute update tree task")?;

    COORDINATOR
        .execute_waiting(AlbumTask::new(album_id))
        .await
        .map_err(anyhow::Error::from)?
        .context("Album task execution failed")?;

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
