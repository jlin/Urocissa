use std::path::PathBuf;
pub mod databaser;
mod filter;
mod importer;
use crate::indexer;
use crate::looper::{LOOPER, Signal};
use crate::tui::DASHBOARD;

pub fn indexer(path: PathBuf) -> anyhow::Result<()> {
    let database = indexer::filter::filter(&path)?;
    DASHBOARD.write().unwrap().add_task(database.hash, path);
    importer::import(&database)?;
    indexer::databaser::databaser(database)?;
    LOOPER.notify(Signal::UpdateTree);
    Ok(())
}
