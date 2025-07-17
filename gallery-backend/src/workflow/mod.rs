use crate::{
    public::constant::runtime::TOKIO_RUNTIME,
    tasks::actor::{
        copy::CopyTask, deduplicate::DeduplicateTask, delete::DeleteTask, index::IndexTask,
        video::VideoTask,
    },
};
use anyhow::Result;
use mini_actor::Actor;
use std::path::PathBuf;
use std::sync::LazyLock;

static COORDINATOR: LazyLock<Actor> = LazyLock::new(|| Actor::new(&TOKIO_RUNTIME));

pub async fn index_for_watch(path: PathBuf) -> Result<()> {
    let database_opt = COORDINATOR
        .execute_waiting(DeduplicateTask::new(path.clone()))
        .await??;
    match database_opt {
        Some(database) => {
            let database = COORDINATOR
                .execute_waiting(CopyTask::new(database))
                .await??;
            let database = COORDINATOR
                .execute_waiting(IndexTask::new(database))
                .await??;

            COORDINATOR.execute_detached(DeleteTask::new(PathBuf::from(path)));

            if database.ext_type == "video" {
                COORDINATOR
                    .execute_waiting(VideoTask::new(database))
                    .await??;
            }
        }
        None => {
            COORDINATOR.execute_detached(DeleteTask::new(path));
        }
    }

    Ok(())
}
