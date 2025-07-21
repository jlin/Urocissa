use anyhow::anyhow;
use anyhow::{Context, Result};
use arrayvec::ArrayString;
use rand::Rng;
use rand::distr::Alphanumeric;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use rocket::post;
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};
use std::time::Instant;

use crate::operations::hash::generate_random_hash;
use crate::operations::open_db::{hash_to_database, index_to_hash, open_data_table};
use crate::public::constant::redb::ALBUM_TABLE;

use crate::public::structure::abstract_data::AbstractData;
use crate::tasks::actor::album::AlbumTask;

use crate::public::db::tree::TREE;
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

#[post("/post/create_non_empty_album", data = "<create_album>")]
pub async fn create_non_empty_album(
    _auth: GuardAuth,
    _read_only_mode: GuardReadOnlyMode,
    create_album: Json<CreateAlbum>,
) -> AppResult<String> {
    let id = tokio::task::spawn_blocking(move || -> Result<ArrayString<64>> {
        let start_time = Instant::now();

        let create_album = create_album.into_inner();
        let album_id = generate_random_hash();
        let album_database = Album::new(album_id, create_album.title);
        let timestamp = &create_album.timestamp;
        let tree_snapshot = TREE_SNAPSHOT
            .read_tree_snapshot(timestamp)
            .context("Failed to read tree snapshot")?;
        let data_table = open_data_table();
        let abstract_data_album = AbstractData::Album(album_database);
        COORDINATOR.execute_batch_detached(FlushTreeTask::insert(vec![abstract_data_album]));
        create_album
            .elements_index
            .into_par_iter()
            .try_for_each(|index| -> Result<()> {
                let hash = index_to_hash(&tree_snapshot, index)
                    .map_err(|e| anyhow!("Failed to read hash by index {}: {}", index, e))?;

                let data = hash_to_database(&data_table, hash)
                    .map_err(|e| anyhow!("Failed to read abstract data by hash {}: {}", hash, e))?;

                let abstract_data = AbstractData::Database(data);

                COORDINATOR.execute_batch_detached(FlushTreeTask::insert(vec![abstract_data]));

                Ok(())
            })?;

        info!(duration = &*format!("{:?}", start_time.elapsed()); "Create album");
        Ok(album_id)
    })
    .await
    .context("Failed to execute blocking task")?
    .context("Task execution failed")?;

    COORDINATOR
        .execute_batch_waiting(UpdateTreeTask)
        .await
        .context("Failed to execute update tree task")?;

    COORDINATOR
        .execute_waiting(AlbumTask::new(id))
        .await
        .map_err(anyhow::Error::from)?
        .context("Album task execution failed")?;

    Ok(id.to_string())
}

#[post("/post/create_empty_album")]
pub async fn create_empty_album(
    _auth: GuardAuth,
    _read_only_mode: GuardReadOnlyMode,
) -> AppResult<String> {
    let id = tokio::task::spawn_blocking(move || -> Result<ArrayString<64>> {
        let start_time = Instant::now();

        let mut album_id = ArrayString::<64>::new();

        for ch in rand::rng()
            .sample_iter(&Alphanumeric)
            .filter(|c| c.is_ascii_lowercase() || c.is_ascii_digit())
            .take(64)
            .map(char::from)
        {
            album_id
                .try_push(ch)
                .context("Failed to push character to album ID")?;
        }

        let album_database = Album::new(album_id, None);
        let txn = TREE
            .in_disk
            .begin_write()
            .context("Failed to begin write transaction")?;

        {
            let mut album_table = txn
                .open_table(ALBUM_TABLE)
                .context("Failed to open album table")?;

            album_table
                .insert(album_id.as_str(), &album_database)
                .context("Failed to insert album into album table")?;
        }
        txn.commit().context("Failed to commit transaction")?;
        info!(duration = &*format!("{:?}", start_time.elapsed()); "Create album");
        Ok(album_id)
    })
    .await
    .context("Failed to execute blocking task")?
    .context("Task execution failed")?;

    COORDINATOR
        .execute_batch_waiting(UpdateTreeTask)
        .await
        .context("Failed to execute update tree task")?;

    COORDINATOR
        .execute_waiting(AlbumTask::new(id))
        .await
        .map_err(anyhow::Error::from)?
        .context("Album task execution failed")?;

    Ok(id.to_string())
}
