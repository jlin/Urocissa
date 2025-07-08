use anyhow::Result;
use std::path::PathBuf;
#[derive(Debug)]
pub struct DeleteTask {
    pub path: PathBuf,
}

pub fn delete_task(task: DeleteTask) -> Result<()> {
    todo!()
}
