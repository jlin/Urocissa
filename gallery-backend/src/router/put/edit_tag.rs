use crate::public::{tree::TREE, tree_snapshot::TREE_SNAPSHOT};

use crate::public::redb::{ALBUM_TABLE, DATA_TABLE};
use crate::public::tree::read_tags::TagInfo;
use crate::public::tree::start_loop::SHOULD_RESET;
use crate::router::fairing::{AuthGuard, ReadOnlyModeGuard};

use redb::ReadableTable;
use rocket::serde::{json::Json, Deserialize};
#[derive(Debug, Deserialize)]
pub struct EditTagsData {
    #[serde(rename = "indexArray")]
    edit_mode_collection_array: Vec<usize>,
    #[serde(rename = "addTagsArray")]
    add_tags_content: Vec<String>,
    #[serde(rename = "removeTagsArray")]
    remove_tags_content: Vec<String>,
    timestamp: u128,
}
#[put("/put/edit_tag", format = "json", data = "<json_data>")]
pub async fn edit_tag(
    _auth: AuthGuard,
    _read_only_mode: ReadOnlyModeGuard,
    json_data: Json<EditTagsData>,
) -> Json<Vec<TagInfo>> {
    let vec_tags_info = tokio::task::spawn_blocking(move || {
        let txn = TREE.in_disk.begin_write().unwrap();
        {
            let mut write_table = txn.open_table(DATA_TABLE).unwrap();
            let mut album_table = txn.open_table(ALBUM_TABLE).unwrap();
            let timestamp = &json_data.timestamp;
            let tree_snapshot = TREE_SNAPSHOT.read_tree_snapshot(timestamp).unwrap();

            json_data
                .edit_mode_collection_array
                .iter()
                .for_each(|index| {
                    let hash = tree_snapshot.get_hash(*index);
                    let data_opt = match write_table.get(hash.as_str()).unwrap() {
                        Some(data) => {
                            let mut data = data.value();
                            json_data.add_tags_content.iter().for_each(|tag| {
                                data.tag.insert(tag.clone());
                            });
                            json_data.remove_tags_content.iter().for_each(|tag| {
                                data.tag.remove(tag);
                            });
                            Some(data)
                        }
                        None => None,
                    };

                    if let Some(data) = data_opt {
                        write_table.insert(&*data.hash, &data).unwrap();
                        return;
                    }

                    let album_opt = match album_table.get(hash.as_str()).unwrap() {
                        Some(data) => {
                            let mut data = data.value();
                            json_data.add_tags_content.iter().for_each(|tag| {
                                data.tag.insert(tag.clone());
                            });
                            json_data.remove_tags_content.iter().for_each(|tag| {
                                data.tag.remove(tag);
                            });
                            Some(data)
                        }
                        None => None,
                    };
                    if let Some(album) = album_opt {
                        album_table.insert(&*album.id, &album).unwrap();
                        return;
                    }
                });
        }
        txn.commit().unwrap();
        let vec_tags_info = TREE_SNAPSHOT.read_tags();
        vec_tags_info
    })
    .await
    .unwrap();
    TREE.should_update().await;
    Json(vec_tags_info)
}
