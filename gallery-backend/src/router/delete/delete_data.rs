use crate::public::constant::redb::{ALBUM_TABLE, DATA_TABLE};

use crate::tasks::actor::album::AlbumTask;
use crate::tasks::batcher::update_tree::UPDATE_TREE_QUEUE;
use crate::tasks::COORDINATOR;

use crate::public::db::tree::TREE;
use crate::public::db::tree_snapshot::TREE_SNAPSHOT;
use crate::router::fairing::guard_auth::GuardAuth;
use crate::router::fairing::guard_read_only_mode::GuardReadOnlyMode;

use futures::future::join_all;
use redb::ReadableTable;
use rocket::serde::{Deserialize, json::Json};
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteList {
    delete_list: Vec<usize>, // Keep this field private as it does not need to be accessed outside this struct
    timestamp: u128,
}
#[delete("/delete/delete-data", format = "json", data = "<json_data>")]
pub async fn delete_data(
    _auth: GuardAuth,
    _read_only_mode: GuardReadOnlyMode,
    json_data: Json<DeleteList>,
) {
    let deleted_album_id = tokio::task::spawn_blocking(move || {
        let timestamp = &json_data.timestamp;

        let tree_snapshot = TREE_SNAPSHOT.read_tree_snapshot(timestamp).unwrap();

        let txn = TREE.in_disk.begin_write().unwrap();
        let mut deleted_album_id = vec![];
        {
            let mut table = txn.open_table(DATA_TABLE).unwrap();
            let mut album_table = txn.open_table(ALBUM_TABLE).unwrap();

            json_data.delete_list.iter().for_each(|index| {
                let hash = tree_snapshot.get_hash(*index);

                let found_data = match table.get(hash.as_str()).unwrap() {
                    Some(data) => {
                        let data = data.value();
                        let compressed_path = data.compressed_path();
                        let imported_path = data.imported_path();
                        std::fs::remove_file(&compressed_path).unwrap_or_else(|err| {
                            error!("Failed to delete file at {:?}: {:#?}", compressed_path, err);
                        });

                        std::fs::remove_file(&imported_path).unwrap_or_else(|err| {
                            error!("Failed to delete file at {:?}: {:#?}", imported_path, err);
                        });

                        for album_id in data.album {
                            deleted_album_id.push(album_id);
                        }

                        true
                    }
                    None => false,
                };
                if found_data {
                    table.remove(hash.as_str()).unwrap();
                }

                let found_album = match album_table.get(hash.as_str()).unwrap() {
                    Some(album) => {
                        let album = album.value();
                        deleted_album_id.push(album.id);
                        true
                    }
                    None => false,
                };
                if found_album {
                    album_table.remove(hash.as_str()).unwrap();
                }
            });
        }

        txn.commit().unwrap();
        deleted_album_id
    })
    .await
    .unwrap();

    UPDATE_TREE_QUEUE.update_async(vec![()]).await;
    let futures = deleted_album_id
        .into_iter()
        .map(async |album_id| COORDINATOR.execute_waiting(AlbumTask::new(album_id)).await);
    join_all(futures).await;
}
