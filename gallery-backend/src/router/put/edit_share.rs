use crate::looper::tree::TREE;
use crate::looper::{LOOPER, Signal};
use crate::structure::album::Share;

use crate::constant::redb::ALBUM_TABLE;
use crate::router::fairing::guard_auth::GuardAuth;
use crate::router::fairing::guard_read_only_mode::GuardReadOnlyMode;

use arrayvec::ArrayString;
use redb::ReadableTable;
use rocket::serde::{Deserialize, json::Json};
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EditShare {
    album_id: ArrayString<64>,
    share: Share,
}

#[put("/put/edit_share", format = "json", data = "<json_data>")]
pub async fn edit_share(
    _auth: GuardAuth,
    _read_only_mode: GuardReadOnlyMode,
    json_data: Json<EditShare>,
) {
    tokio::task::spawn_blocking(move || {
        let txn = TREE.in_disk.begin_write().unwrap();
        {
            let mut album_table = txn.open_table(ALBUM_TABLE).unwrap();

            let album_opt = album_table
                .get(json_data.album_id.as_str())
                .unwrap() // error-check result if needed
                .map(|guard| guard.value());

            if let Some(mut album) = album_opt {
                album
                    .share_list
                    .insert(json_data.share.url, json_data.share.clone());
                album_table
                    .insert(json_data.album_id.as_str(), &album)
                    .unwrap();
            }
        }
        txn.commit().unwrap();
    })
    .await
    .unwrap();

    LOOPER.notify_with_ack(Signal::Update).await.unwrap();
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteShare {
    album_id: ArrayString<64>,
    share_id: ArrayString<64>,
}

#[put("/put/delete_share", format = "json", data = "<json_data>")]
pub async fn delete_share(
    _auth: GuardAuth,
    _read_only_mode: GuardReadOnlyMode,
    json_data: Json<DeleteShare>,
) {
    tokio::task::spawn_blocking(move || {
        let txn = TREE.in_disk.begin_write().unwrap();
        {
            let mut album_table = txn.open_table(ALBUM_TABLE).unwrap();

            let album_opt = album_table
                .get(json_data.album_id.as_str())
                .unwrap() // error-check result if needed
                .map(|guard| guard.value());

            if let Some(mut album) = album_opt {
                album.share_list.remove(&json_data.share_id);
                album_table
                    .insert(json_data.album_id.as_str(), &album)
                    .unwrap();
            }
        }
        txn.commit().unwrap();
    })
    .await
    .unwrap();
    LOOPER.notify_with_ack(Signal::Update).await.unwrap();
}
