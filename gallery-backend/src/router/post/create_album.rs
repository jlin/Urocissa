use std::time::Instant;

use arrayvec::ArrayString;
use rand::Rng;
use rand::distr::Alphanumeric;
use redb::ReadableTable;
use rocket::serde::json::Json;
use rocket::{http::Status, post};

use serde::{Deserialize, Serialize};

use crate::constant::redb::{ALBUM_TABLE, DATA_TABLE};
use crate::coordinator::album::AlbumTask;
use crate::coordinator::{COORDINATOR, Task};
use crate::looper::tree::TREE;
use crate::looper::tree_snapshot::TREE_SNAPSHOT;
use crate::router::fairing::guard_auth::GuardAuth;
use crate::router::fairing::guard_read_only_mode::GuardReadOnlyMode;
use crate::structure::album::Album;

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
) -> Result<String, Status> {
    let id = tokio::task::spawn_blocking(move || {
        let start_time = Instant::now();
        let create_album = create_album.into_inner();
        let album_id: String = rand::rng()
            .sample_iter(&Alphanumeric)
            .filter(|c| c.is_ascii_lowercase() || c.is_ascii_digit())
            .take(64)
            .map(char::from)
            .collect();
        let album_id = ArrayString::<64>::from(&album_id).unwrap();

        let album_database = Album::new(album_id, create_album.title);
        let txn = TREE.in_disk.begin_write().unwrap();

        let timestamp = &create_album.timestamp;
        let tree_snapshot = TREE_SNAPSHOT.read_tree_snapshot(timestamp).unwrap();

        {
            let mut album_table = txn.open_table(ALBUM_TABLE).unwrap();

            album_table
                .insert(album_id.as_str(), &album_database)
                .unwrap();

            let mut data_table = txn.open_table(DATA_TABLE).unwrap();

            create_album.elements_index.iter().for_each(|index| {
                let hash = tree_snapshot.get_hash(*index);

                // album should not be added to album
                let data_opt = data_table.get(hash.as_str()).unwrap().map(|data_guard| {
                    let mut data = data_guard.value();
                    data.album.insert(album_id);
                    data
                });
                if let Some(data) = data_opt {
                    data_table.insert(&*data.hash, &data).unwrap();
                }
            });
        }
        txn.commit().unwrap();
        info!(duration = &*format!("{:?}", start_time.elapsed()); "Create album");
        album_id
    })
    .await
    .unwrap();
    TREE.should_update_async().await;
    COORDINATOR
        .submit_with_ack(Task::Album(AlbumTask::new(id)))
        .unwrap()
        .await;
    Ok(id.to_string())
}

#[post("/post/create_empty_album")]
pub async fn create_empty_album(
    _auth: GuardAuth,
    _read_only_mode: GuardReadOnlyMode,
) -> Result<String, Status> {
    let id = tokio::task::spawn_blocking(move || {
        let start_time = Instant::now();

        let album_id: String = rand::rng()
            .sample_iter(&Alphanumeric)
            .filter(|c| c.is_ascii_lowercase() || c.is_ascii_digit())
            .take(64)
            .map(char::from)
            .collect();
        let album_id = ArrayString::<64>::from(&album_id).unwrap();

        let album_database = Album::new(album_id, None);
        let txn = TREE.in_disk.begin_write().unwrap();

        {
            let mut album_table = txn.open_table(ALBUM_TABLE).unwrap();

            album_table
                .insert(album_id.as_str(), &album_database)
                .unwrap();
        }
        txn.commit().unwrap();
        info!(duration = &*format!("{:?}", start_time.elapsed()); "Create album");
        album_id
    })
    .await
    .unwrap();
    TREE.should_update_async().await;
    COORDINATOR
        .submit_with_ack(Task::Album(AlbumTask::new(id)))
        .unwrap()
        .await;
    Ok(id.to_string())
}
