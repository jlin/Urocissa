use std::fs;

use anyhow::Context;

use crate::{
    coordinator::{COORDINATOR, Task, index::IndexTask},
    structure::database_struct::database::definition::Database,
};

#[derive(Debug)]
pub struct CopyTask {
    pub database: Database,
}
impl CopyTask {
    pub fn new(database: Database) -> Self {
        Self { database }
    }
}

pub fn copy_task(task: CopyTask) -> anyhow::Result<()> {
    let database = task.database;
    let source_path = database.source_path();
    let dest_path = database.imported_path();

    if let Some(parent) = dest_path.parent() {
        fs::create_dir_all(parent).context(format!("failed to create directory: {parent:?}"))?;
    }

    fs::copy(&source_path, &dest_path)
        .context(format!("failed to copy {source_path:?} → {dest_path:?}"))?;
    COORDINATOR.submit(Task::Index(IndexTask::new(database)))?;
    Ok(())
}
