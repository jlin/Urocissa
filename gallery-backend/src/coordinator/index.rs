use anyhow::Result;
use std::path::PathBuf;

use crate::executor::executor;

#[derive(Debug)]
pub struct IndexTask {
    pub path: PathBuf,
}
impl IndexTask {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }
}

pub fn index_task(task: IndexTask) -> Result<()> {
    executor(task.path)?;
    Ok(())
}
