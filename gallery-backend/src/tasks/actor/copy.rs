use anyhow::Context;
use anyhow::Error;
use anyhow::Result;
use mini_executor::Task;
use std::fs;
use std::io;
use std::path::Path;
use std::thread;
use std::time::Duration;
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

    fn run(self) -> impl Future<Output = Self::Output> + Send {
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

    if let Some(parent) = dest_path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("failed to create directory tree for {:?}", parent))?;
    }

    robust_copy(&source_path, &dest_path).with_context(|| {
        format!(
            "failed to copy file from {:?} to {:?}",
            source_path, dest_path
        )
    })?; // If it fails three times, it goes into the Err branch

    Ok(database)
}

fn robust_copy(src: &Path, dst: &Path) -> io::Result<u64> {
    const MAX_RETRIES: u32 = 3;

    for attempt in 0..=MAX_RETRIES {
        match fs::copy(src, dst) {
            Ok(bytes) => return Ok(bytes), // On success, exit early
            Err(e) if attempt < MAX_RETRIES => {
                warn!(
                    "fs::copy({:?} → {:?}) failed on attempt {}/{}:\n{:?}. Retrying in 1 s",
                    src,
                    dst,
                    attempt + 1,
                    MAX_RETRIES + 1,
                    Error::new(e)
                );
                thread::sleep(Duration::from_secs(1)); // Block for 1 second
                continue; // Continue to the next attempt
            }
            Err(e) => return Err(e), // If the 4th attempt still fails, return the error
        }
    }
    unreachable!("loop guarantees return")
}
