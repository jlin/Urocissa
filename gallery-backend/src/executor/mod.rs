use std::path::PathBuf;
mod batcher;
pub mod databaser;
mod filter;
mod importer;
use crate::looper::tree::TREE;
use crate::{constant::PROCESS_BATCH_NUMBER, executor};
use anyhow::Result;
use batcher::merge_file_paths;

pub fn executor(path: PathBuf) -> Result<()> {
    processor(path)?;
    TREE.tree_update();
    Ok(())
}

fn processor(path: PathBuf) -> Result<()> {
    let database = executor::filter::filter(path)?;
    importer::import(&database).unwrap();
    executor::databaser::databaser(database);
    Ok(())
}
