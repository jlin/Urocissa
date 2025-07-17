use crate::tasks::{
    COORDINATOR,
    actor::{
        copy::CopyTask, deduplicate::DeduplicateTask, delete_in_update::DeleteTask,
        index::IndexTask, video::VideoTask,
    },
};
use anyhow::{Result, bail};
use arrayvec::ArrayString;
use dashmap::DashSet;
use std::{path::PathBuf, sync::LazyLock};

static IN_PROGRESS: LazyLock<DashSet<ArrayString<64>>> = LazyLock::new(DashSet::new);

pub struct ProcessingGuard(ArrayString<64>);

impl ProcessingGuard {
    /// 嘗試取得鎖（第一次插入成功），否則回傳 `None`
    pub fn try_acquire(hash: ArrayString<64>) -> Option<Self> {
        if IN_PROGRESS.insert(hash) {
            Some(Self(hash))
        } else {
            None
        }
    }
}

/// 離開作用域時自動解鎖
impl Drop for ProcessingGuard {
    fn drop(&mut self) {
        IN_PROGRESS.remove(&self.0);
    }
}

pub async fn index_for_watch(path: PathBuf) -> Result<()> {
    let database_opt = COORDINATOR
        .execute_waiting(DeduplicateTask::new(path.clone()))
        .await??;
    match database_opt {
        Some(database) => {
            if let Some(g) = ProcessingGuard::try_acquire(database.hash) {
                let database = COORDINATOR
                    .execute_waiting(CopyTask::new(database))
                    .await??;

                let database = COORDINATOR
                    .execute_waiting(IndexTask::new(database))
                    .await??;

                COORDINATOR.execute_detached(DeleteTask::new(PathBuf::from(&path)));

                if database.ext_type == "video" {
                    COORDINATOR
                        .execute_waiting(VideoTask::new(database))
                        .await??;
                }
            } else {
                bail!("File is already being processed: {:#?}", database);
            }
        }
        None => {
            COORDINATOR.execute_detached(DeleteTask::new(path));
        }
    }

    Ok(())
}
