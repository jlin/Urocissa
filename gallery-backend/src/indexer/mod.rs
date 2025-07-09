use std::path::PathBuf;
pub mod databaser;
mod filter;
mod importer;
use crate::coordinator::{COORDINATOR, Task};
use crate::indexer;

pub fn indexer(path: PathBuf) -> anyhow::Result<()> {
    let database = indexer::filter::filter(path)?;
    importer::import(&database)?;
    indexer::databaser::databaser(database)?;
    COORDINATOR.submit(Task::Update).unwrap();
    Ok(())
}
