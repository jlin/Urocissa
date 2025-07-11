use std::path::PathBuf;
pub mod databaser;
mod filter;
mod importer;
use crate::indexer;
use crate::looper::{LOOPER, Signal};
use crate::tui::{DASHBOARD, FileType};

pub fn indexer(path: PathBuf) -> anyhow::Result<()> {
    let database = indexer::filter::filter(&path)?;
    let hash = database.hash;
    DASHBOARD.write().unwrap().add_task(
        hash,
        path,
        FileType::try_from(database.ext_type.as_str())?,
    );
    importer::import(&database)?;
    indexer::databaser::databaser(database)?;
    LOOPER.notify(Signal::UpdateTree);
    DASHBOARD.write().unwrap().advance_task_state(&hash);

    Ok(())
}
