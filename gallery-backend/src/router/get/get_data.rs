use crate::public::config::{PUBLIC_CONFIG, PublicConfig};
use crate::public::tree::TREE;
use crate::public::tree::read_tags::TagInfo;
use crate::router::fairing::guard_auth::GuardAuth;

use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};
use std::hash::Hash;

#[get("/get/get-config.json")]
pub async fn get_config(_auth: GuardAuth) -> Json<&'static PublicConfig> {
    Json(&*PUBLIC_CONFIG)
}

#[get("/get/get-tags")]
pub async fn get_tags(_auth: GuardAuth) -> Json<Vec<TagInfo>> {
    tokio::task::spawn_blocking(move || {
        let vec_tags_info = TREE.read_tags();
        Json(vec_tags_info)
    })
    .await
    .unwrap()
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct AlbumInfo {
    pub album_id: String,
    pub album_name: Option<String>,
}

#[get("/get/get-albums")]
pub async fn get_albums(_auth: GuardAuth) -> Json<Vec<AlbumInfo>> {
    tokio::task::spawn_blocking(move || {
        let album_list = TREE.read_albums();
        let album_info_list = album_list
            .into_iter()
            .map(|album| AlbumInfo {
                album_id: album.id.to_string(),
                album_name: album.title,
            })
            .collect();
        Json(album_info_list)
    })
    .await
    .unwrap()
}
