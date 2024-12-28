use std::sync::Arc;
use std::time::Instant;

use arrayvec::ArrayString;
use rand::distributions::Alphanumeric;
use rand::Rng;
use redb::ReadableTable;
use rocket::serde::json::Json;
use rocket::{http::Status, post};

use serde::{Deserialize, Serialize};
use tokio::sync::Notify;

use crate::public::album::Album;
use crate::public::redb::{ALBUM_TABLE, DATA_TABLE};
use crate::public::tree::start_loop::{ALBUM_WAITING_FOR_MEMORY_UPDATE_SENDER, SHOULD_RESET};
use crate::public::tree::TREE;
use crate::router::fairing::{AuthGuard, ReadOnlyModeGuard};
use crate::router::put::edit_album::AlbumQueue;

#[derive(Debug, Clone, Deserialize, Default, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CreateAlbum {
    pub title: Option<String>,
    pub elements: Vec<ArrayString<64>>,
}

#[post("/post/create_album", data = "<create_album>")]
pub async fn create_album(
    _auth: AuthGuard,
    _read_only_mode: ReadOnlyModeGuard,
    create_album: Json<CreateAlbum>,
) -> Result<String, Status> {
    let id = tokio::task::spawn_blocking(move || {
        let start_time = Instant::now();
        let create_album = create_album.into_inner();
        let album_id: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .filter(|c| c.is_ascii_lowercase() || c.is_ascii_digit())
            .take(64)
            .map(char::from)
            .collect();
        let album_id = ArrayString::<64>::from(&album_id).unwrap();

        let album_database = Album::new(album_id, create_album.title);
        let txn = TREE.in_disk.begin_write().unwrap();
        {
            let mut album_table = txn.open_table(ALBUM_TABLE).unwrap();

            album_table
                .insert(album_id.as_str(), &album_database)
                .unwrap();

            let mut data_table = txn.open_table(DATA_TABLE).unwrap();

            create_album.elements.iter().for_each(|hash| {
                let mut data = data_table.get(hash.as_str()).unwrap().unwrap().value();
                data.album.insert(album_id);

                data_table.insert(&*data.hash, &data).unwrap();
            });
        }
        txn.commit().unwrap();
        info!(duration = &*format!("{:?}", start_time.elapsed()); "Create album");
        album_id
    })
    .await
    .unwrap();
    let waiting_update = Arc::new(Notify::new());
    let album_queue = AlbumQueue {
        album_list: vec![id],
        notify: Some(Arc::clone(&waiting_update)),
    };
    ALBUM_WAITING_FOR_MEMORY_UPDATE_SENDER
        .get()
        .unwrap()
        .send(album_queue)
        .unwrap();

    SHOULD_RESET.notify_one();
    waiting_update.notified().await;
    Ok(id.to_string())
}
