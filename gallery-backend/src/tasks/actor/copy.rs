use anyhow::Context;
use anyhow::Result;
use mini_actor::Task;
use std::fs;
use tokio::task::spawn_blocking;

use crate::{
    public::structure::database_struct::database::definition::Database,
    tasks::{COORDINATOR, actor::index::IndexTask},
};

pub struct CopyTask {
    pub database: Database,
}

impl CopyTask {
    pub fn new(database: Database) -> Self {
        Self { database }
    }
}

impl Task for CopyTask {
    type Output = Result<()>;

    fn run(self) -> impl std::future::Future<Output = Self::Output> + Send {
        async move {
            let result = spawn_blocking(move || copy_task(self.database))
                .await
                .expect("blocking task panicked");
            result
        }
    }
}

pub fn copy_task(database: Database) -> Result<()> {
    let source_path = database.source_path();
    let dest_path = database.imported_path();

    if let Some(parent) = dest_path.parent() {
        fs::create_dir_all(parent).context(format!("failed to create directory: {parent:?}"))?;
    }

    fs::copy(&source_path, &dest_path)
        .context(format!("failed to copy {source_path:?} → {dest_path:?}"))?;
    COORDINATOR.execute_detached(IndexTask::new(database));
    Ok(())
}
