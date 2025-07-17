use crate::tasks::{
    COORDINATOR,
    actor::{copy::CopyTask, deduplicate::DeduplicateTask, index::IndexTask},
};
use anyhow::Result;
use std::path::PathBuf;

pub async fn index_for_watch(path: PathBuf) -> Result<()> {
    let database = COORDINATOR
        .execute_waiting(DeduplicateTask::new(path))
        .await??;
    let database = COORDINATOR
        .execute_waiting(CopyTask::new(database))
        .await??;
    COORDINATOR
        .execute_waiting(IndexTask::new(database))
        .await??;

    Ok(())
}
