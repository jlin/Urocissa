use std::path::PathBuf;
mod batcher;
pub mod databaser;
mod filter;
mod importer;
use crate::looper::tree::TREE;
use crate::{constant::PROCESS_BATCH_NUMBER, executor};
use anyhow::Result;
use batcher::merge_file_paths;

pub fn executor(list_of_sync_files: Vec<PathBuf>) {
    let all_paths = merge_file_paths(list_of_sync_files);
    let total_batches = (all_paths.len() + PROCESS_BATCH_NUMBER - 1) / PROCESS_BATCH_NUMBER; // Calculate total number of batches
    for (current_batch, batch) in all_paths.chunks(PROCESS_BATCH_NUMBER).enumerate() {
        info!("Processing batch {}/{}", current_batch + 1, total_batches); // Show the current batch being processed
        let batch: Vec<PathBuf> = batch.to_vec();
        let successfully_handled_length = processor(batch);
        if successfully_handled_length > 0 {
            TREE.tree_update();
        }
    }
}

fn processor(path: PathBuf) -> Result<()> {
    let database = executor::filter::filter(path)?;
    importer::import(&database).unwrap();
    executor::databaser::databaser(database);
    Ok(())
}
