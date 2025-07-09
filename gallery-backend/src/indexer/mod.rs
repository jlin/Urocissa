use std::path::PathBuf;
pub mod databaser;
mod filter;
mod importer;
use crate::indexer;
use crate::looper::tree::TREE;
use anyhow::Result;

pub fn indexer(path: PathBuf) -> Result<()> {
    let database = indexer::filter::filter(path)?;
    importer::import(&database)?;
    indexer::databaser::databaser(database)?;
    TREE.tree_update();
    Ok(())
}
