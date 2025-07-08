use std::path::PathBuf;
mod batcher;
pub mod databaser;
mod filter;
mod importer;
use crate::executor;
use crate::looper::tree::TREE;
use anyhow::Result;

pub fn executor(path: PathBuf) -> Result<()> {
    processor(path)?;
    TREE.tree_update();
    Ok(())
}

fn processor(path: PathBuf) -> Result<()> {
    let database = executor::filter::filter(path)?;
    importer::import(&database)?;
    executor::databaser::databaser(database)?;
    Ok(())
}
