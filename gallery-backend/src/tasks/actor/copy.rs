use anyhow::Context;
use anyhow::Result;
use mini_coordinator::Task;
use std::fs;
use tokio::task::spawn_blocking;

use crate::public::error_data::handle_error;
use crate::public::structure::database_struct::database::definition::Database;

pub struct CopyTask {
    pub database: Database,
}

impl CopyTask {
    pub fn new(database: Database) -> Self {
        Self { database }
    }
}

impl Task for CopyTask {
    type Output = Result<Database>;

    fn run(self) -> impl std::future::Future<Output = Self::Output> + Send {
        async move {
            spawn_blocking(move || copy_task(self.database))
                .await
                .expect("blocking task panicked")
                .map_err(|err| handle_error(err.context("Failed to run copy task")))
        }
    }
}

pub fn copy_task(database: Database) -> Result<Database> {
    let source_path = database.source_path();
    let dest_path = database.imported_path();

    // Make sure the parent directory tree exists
    if let Some(parent) = dest_path.parent() {
        fs::create_dir_all(parent).context(format!(
            "failed to create directory tree for destination {:?}",
            parent
        ))?;
    }

    // Perform the copy
    fs::copy(&source_path, &dest_path).context(format!(
        "failed to copy file from {:?} to {:?}",
        source_path, dest_path
    ))?;

    Ok(database)
}
