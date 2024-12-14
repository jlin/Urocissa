use std::path::PathBuf;
mod batcher;
pub mod compressor;
mod databaser;
mod filter;
mod importer;
use crate::{executor, public::tree::start_loop::SHOULD_RESET, synchronizer::event::BATCH_SIZE};
use batcher::merge_file_paths;

pub fn executor(list_of_sync_files: Vec<PathBuf>) {
    let all_paths = merge_file_paths(list_of_sync_files);
    let total_batches = (all_paths.len() + BATCH_SIZE - 1) / BATCH_SIZE; // Calculate total number of batches
    for (current_batch, batch) in all_paths.chunks(BATCH_SIZE).enumerate() {
        info!("Processing batch {}/{}", current_batch + 1, total_batches); // Show the current batch being processed
        let batch: Vec<PathBuf> = batch.to_vec();
        processor(batch);
        SHOULD_RESET.notify_one();
    }
}

fn processor(list_of_sync_files: Vec<PathBuf>) {
    let deduplicated_file_list = executor::filter::filter(list_of_sync_files);
    importer::import(&deduplicated_file_list).unwrap();
    let database = executor::databaser::databaser(deduplicated_file_list);
    executor::compressor::compressor(database);
}
