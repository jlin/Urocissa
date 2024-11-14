use crate::public::abstract_data::AbstractData;
use crate::public::database_struct::database::definition::DataBase;
use crate::public::redb::{ALBUM_TABLE, DATA_TABLE};
use crate::public::tree::start_loop::SHOULD_RESET;
use crate::public::tree::TREE;
use crate::public::tree_snapshot::TREE_SNAPSHOT;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use redb::ReadableTable;
use rocket::serde::{json::Json, Deserialize};
use std::sync::atomic::Ordering;
#[derive(Debug, Deserialize)]
pub struct DeleteList {
    #[serde(rename = "deleteList")]
    delete_list: Vec<usize>, // Keep this field private as it does not need to be accessed outside this struct
    timestamp: String,
}
#[delete("/delete/delete-data", format = "json", data = "<json_data>")]
pub async fn delete_data(json_data: Json<DeleteList>) {
    tokio::task::spawn_blocking(move || {
        let timestamp = &json_data.timestamp;

        let tree_snapshot = TREE_SNAPSHOT.read_tree_snapshot(timestamp).unwrap();

        let txn = TREE.in_disk.begin_write().unwrap();

        {
            let mut table = txn.open_table(DATA_TABLE).unwrap();

            for index in &json_data.delete_list {
                let hash = tree_snapshot.get_hash(*index);

                let found_data = match table.get(hash.as_str()).unwrap() {
                    Some(data) => {
                        let data = data.value();
                        let compressed_path = data.compressed_path();
                        let imported_path = data.imported_path();
                        std::fs::remove_file(&compressed_path).unwrap_or_else(|err| {
                            error!("Failed to delete file at {:?}: {:?}", compressed_path, err);
                        });

                        std::fs::remove_file(&imported_path).unwrap_or_else(|err| {
                            error!("Failed to delete file at {:?}: {:?}", imported_path, err);
                        });
                        true
                    }
                    None => false,
                };
                if found_data {
                    table.remove(hash.as_str()).unwrap();
                    return;
                }

                let mut album_table = txn.open_table(ALBUM_TABLE).unwrap();

                match album_table.get(hash.as_str()).unwrap() {
                    Some(data) => {
                        let album = data.value();
                        let ref_data = TREE.in_memory.read().unwrap();
                        ref_data.iter().for_each(|database_timestamp| {
                            match &database_timestamp.abstract_data {
                                AbstractData::DataBase(database) => {
                                    if database.album.contains(&album.id) {
                                        let mut database = database.clone();
                                        database.album.remove(&album.id);
                                        table.insert(&*database.hash.clone(), database).unwrap();
                                    }
                                }
                                AbstractData::Album(_) => (),
                            }
                        });
                    }
                    None => (),
                };
                if found_data {
                    table.remove(hash.as_str()).unwrap();
                }
            }
        }

        txn.commit().unwrap();
        SHOULD_RESET.store(true, Ordering::SeqCst);
    })
    .await
    .unwrap();
}
