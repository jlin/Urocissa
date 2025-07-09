use std::path::PathBuf;
pub mod databaser;
mod filter;
mod importer;
use crate::indexer;
use crate::looper::tree::TREE;

pub fn indexer(path: PathBuf) -> anyhow::Result<()> {
    let database = indexer::filter::filter(path)?;
    importer::import(&database)?;
    indexer::databaser::databaser(database)?;
    TREE.tree_update();
    Ok(())
}
