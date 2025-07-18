use crate::tasks::{
    COORDINATOR,
    actor::{
        copy::CopyTask, deduplicate::DeduplicateTask, delete_in_update::DeleteTask, hash::HashTask,
        index::IndexTask, open_file::OpenFileTask, video::VideoTask,
    },
};
use anyhow::{Result, bail};
use arrayvec::ArrayString;
use dashmap::DashSet;
use log::{info, warn};
use path_clean::PathClean;
use std::{path::PathBuf, sync::LazyLock};

static IN_PROGRESS: LazyLock<DashSet<ArrayString<64>>> = LazyLock::new(DashSet::new);

pub struct ProcessingGuard(ArrayString<64>);
impl Drop for ProcessingGuard {
    fn drop(&mut self) {
        IN_PROGRESS.remove(&self.0);
    }
}

fn try_acquire(hash: ArrayString<64>) -> Option<ProcessingGuard> {
    if IN_PROGRESS.insert(hash.clone()) {
        Some(ProcessingGuard(hash))
    } else {
        None
    }
}

pub async fn index_for_watch(path: PathBuf) -> Result<()> {
    let path = path.clean();
    let file = COORDINATOR
        .execute_waiting(OpenFileTask::new(path.clone()))
        .await??;
    let hash = COORDINATOR.execute_waiting(HashTask::new(file)).await??;

    let _guard = match try_acquire(hash) {
        Some(g) => g,
        None => {
            warn!(
                "Processing already in progress for path: {:?}, hash: {}",
                path, hash
            );
            bail!(
                "Processing already in progress for path: {:?}, hash: {}",
                path,
                hash
            );
        }
    };

    let database_opt = COORDINATOR
        .execute_waiting(DeduplicateTask::new(path.clone(), hash))
        .await??;

    // If the file is already in the database, we can skip further processing.
    let mut database = match database_opt {
        Some(db) => db,
        None => {
            COORDINATOR.execute_detached(DeleteTask::new(path));
            return Ok(());
        }
    };

    database = COORDINATOR
        .execute_waiting(CopyTask::new(database))
        .await??;
    database = COORDINATOR
        .execute_waiting(IndexTask::new(database))
        .await??;

    COORDINATOR.execute_detached(DeleteTask::new(PathBuf::from(&path)));
    info!(
        "Ready to processed video file: {:?}, hash: {}",
        path, database.hash
    );
    if database.ext_type == "video" {
        COORDINATOR
            .execute_waiting(VideoTask::new(database))
            .await??;
    }

    Ok(())
}
