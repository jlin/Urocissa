use std::path::PathBuf;
pub mod databaser;
mod filter;
mod importer;
use crate::executor;
use crate::looper::tree::TREE;
use anyhow::Result;

pub fn executor(path: PathBuf) -> Result<()> {
    let database = executor::filter::filter(path)?;
    importer::import(&database)?;
    executor::databaser::databaser(database)?;
    TREE.tree_update();
    Ok(())
}
