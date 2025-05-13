use std::path::PathBuf;
mod batcher;
pub mod databaser;
mod filter;
mod importer;
use crate::looper::tree::TREE;
use crate::{executor, public::constant::PROCESS_BATCH_NUMBER};
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

fn processor(list_of_sync_files: Vec<PathBuf>) -> usize {
    let deduplicated_file_list = executor::filter::filter(list_of_sync_files);
    importer::import(&deduplicated_file_list).unwrap();
    let successfully_handled_length = executor::databaser::databaser(deduplicated_file_list);
    successfully_handled_length
}
