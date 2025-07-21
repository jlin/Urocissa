use crate::operations::open_db::open_data_and_album_tables;
use crate::process::transitor::index_to_abstract_data;
use crate::public::db::tree_snapshot::TREE_SNAPSHOT;

use crate::public::db::tree::read_tags::TagInfo;
use crate::router::AppResult;
use crate::router::fairing::guard_auth::GuardAuth;
use crate::router::fairing::guard_read_only_mode::GuardReadOnlyMode;
use crate::tasks::COORDINATOR;
use crate::tasks::batcher::flush_tree::FlushTreeTask;
use crate::tasks::batcher::update_tree::UpdateTreeTask;
use anyhow::Result;
use rocket::serde::{Deserialize, json::Json};
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EditTagsData {
    index_array: Vec<usize>,
    add_tags_array: Vec<String>,
    remove_tags_array: Vec<String>,
    timestamp: u128,
}
#[put("/put/edit_tag", format = "json", data = "<json_data>")]
pub async fn edit_tag(
    _auth: GuardAuth,
    _read_only_mode: GuardReadOnlyMode,
    json_data: Json<EditTagsData>,
) -> AppResult<Json<Vec<TagInfo>>> {
    let vec_tags_info = tokio::task::spawn_blocking(move || -> Result<Vec<TagInfo>> {
        let (data_table, album_table) = open_data_and_album_tables();
        let timestamp = &json_data.timestamp;
        let tree_snapshot = TREE_SNAPSHOT.read_tree_snapshot(timestamp).unwrap();

        json_data
            .index_array
            .iter()
            .try_for_each(|index| -> Result<()> {
                let mut abstract_data =
                    index_to_abstract_data(&tree_snapshot, &data_table, &album_table, *index)
                        .map_err(|e| {
                            anyhow::anyhow!("Failed to convert index to abstract data: {}", e)
                        })?;
                let tag_set = abstract_data.tag_mut();
                json_data.add_tags_array.iter().for_each(|tag| {
                    tag_set.insert(tag.clone());
                });
                json_data.remove_tags_array.iter().for_each(|tag| {
                    tag_set.remove(tag);
                });
                COORDINATOR.execute_batch_detached(FlushTreeTask::insert(vec![abstract_data]));
                Ok(())
            })?;

        let vec_tags_info = TREE_SNAPSHOT.read_tags();
        Ok(vec_tags_info)
    })
    .await
    .unwrap()?;
    COORDINATOR
        .execute_batch_waiting(UpdateTreeTask)
        .await
        .unwrap();
    Ok(Json(vec_tags_info))
}
