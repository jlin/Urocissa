use std::collections::HashSet;
use std::sync::atomic::Ordering;

use crate::public::tree::start_loop::{ALBUM_WAITING_FOR_MEMORY_UPDATE_SENDER, SHOULD_RESET};
use crate::public::{tree::TREE, tree_snapshot::TREE_SNAPSHOT};

use crate::public::redb::{ALBUM_TABLE, DATA_TABLE};
use arrayvec::ArrayString;
use log::info;
use redb::ReadableTable;
use rocket::http::Status;
use rocket::serde::{json::Json, Deserialize};
use serde::Serialize;
#[derive(Debug, Deserialize)]
pub struct EditAlbumsData {
    #[serde(rename = "idArray")]
    edit_mode_collection_array: Vec<usize>,
    #[serde(rename = "addAlbumsArray")]
    add_albums_content: Vec<ArrayString<64>>,
    #[serde(rename = "removeAlbumsArray")]
    remove_albums_content: Vec<ArrayString<64>>,
    timestamp: String,
}
#[put("/put/edit_album", format = "json", data = "<json_data>")]
pub async fn edit_album(json_data: Json<EditAlbumsData>) -> () {
    tokio::task::spawn_blocking(move || {
        let txn = TREE.in_disk.begin_write().unwrap();
        {
            let mut write_table = txn.open_table(DATA_TABLE).unwrap();

            let timestamp = &json_data.timestamp;
            let tree_snapshot = TREE_SNAPSHOT.read_tree_snapshot(timestamp).unwrap();

            json_data
                .edit_mode_collection_array
                .iter()
                .for_each(|index| {
                    let hash = tree_snapshot.get_hash(*index);
                    let mut data = write_table.get(hash.as_str()).unwrap().unwrap().value();
                    json_data.add_albums_content.iter().for_each(|album_id| {
                        data.album.insert(album_id.clone());
                    });

                    json_data.remove_albums_content.iter().for_each(|album_id| {
                        data.album.remove(album_id);
                    });

                    write_table.insert(&*data.hash, &data).unwrap();
                });
        }
        txn.commit().unwrap();
        SHOULD_RESET.store(true, Ordering::SeqCst);

        let concact_result: Vec<ArrayString<64>> = json_data
            .add_albums_content
            .iter()
            .chain(json_data.remove_albums_content.iter())
            .cloned()
            .collect::<HashSet<_>>()
            .into_iter()
            .collect();

        ALBUM_WAITING_FOR_MEMORY_UPDATE_SENDER
            .get()
            .unwrap()
            .send(concact_result)
            .unwrap();
        info!("Send concact_result")
    })
    .await
    .unwrap()
}

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
