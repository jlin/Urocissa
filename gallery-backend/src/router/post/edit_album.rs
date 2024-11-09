use std::sync::atomic::Ordering;

use arrayvec::ArrayString;
use redb::ReadableTable;
use rocket::serde::json::Json;
use rocket::{http::Status, post};

use serde::{Deserialize, Serialize};

use crate::public::redb::{ALBUM_TABLE, DATA_TABLE};
use crate::public::tree::start_loop::SHOULD_RESET;
use crate::public::tree::TREE;

#[derive(Debug, Clone, Deserialize, Default, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SetAlbumCover {
    pub album_id: ArrayString<64>,
    pub cover_hash: ArrayString<64>,
}

#[post("/post/set_album_cover", data = "<set_album_cover>")]
pub async fn set_album_cover(set_album_cover: Json<SetAlbumCover>) -> Result<(), Status> {
    tokio::task::spawn_blocking(move || {
        let set_album_cover_inner = set_album_cover.into_inner();
        let album_id = set_album_cover_inner.album_id;
        let cover_hash = set_album_cover_inner.cover_hash;

        let txn = TREE.in_disk.begin_write().unwrap();
        {
            let mut album_table = txn.open_table(ALBUM_TABLE).unwrap();
            let data_table = txn.open_table(DATA_TABLE).unwrap();

            let mut album = album_table.get(&*album_id).unwrap().unwrap().value();
            let database = data_table.get(&*cover_hash).unwrap().unwrap().value();

            album.set_cover(&database);
            album_table.insert(&*album_id, album).unwrap();
        }
        txn.commit().unwrap();
        SHOULD_RESET.swap(true, Ordering::SeqCst);
        Ok(())
    })
    .await
    .unwrap()
}
