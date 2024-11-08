use std::sync::atomic::Ordering;

use crate::public::{tree::TREE, tree_snapshot::TREE_SNAPSHOT};

use crate::public::redb::DATA_TABLE;
use crate::public::tree::read_tags::TagInfo;
use crate::public::tree::start_loop::SHOULD_RESET;
use arrayvec::ArrayString;
use redb::ReadableTable;
use rocket::serde::{json::Json, Deserialize};
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
                    json_data.add_albums_content.iter().for_each(|album| {
                        data.album.insert(album.clone());
                    });
                    json_data.remove_albums_content.iter().for_each(|album| {
                        data.album.remove(album);
                    });

                    write_table.insert(&*data.hash, &data).unwrap();
                });
        }
        txn.commit().unwrap();
        SHOULD_RESET.store(true, Ordering::SeqCst);
    })
    .await
    .unwrap()
}
