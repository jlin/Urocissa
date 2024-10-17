use std::{path::PathBuf, sync::atomic::Ordering};
mod batcher;
pub mod compressor;
mod databaser;
mod filter;
mod importer;
use crate::{executor, public::tree::start_loop::SHOULD_RESET, BATCH_SIZE};
use batcher::merge_file_paths;
use indicatif::{ProgressBar, ProgressDrawTarget, ProgressStyle};

pub fn executor(list_of_sync_files: Vec<PathBuf>) {
    let all_paths = merge_file_paths(list_of_sync_files);
    let total_batches = (all_paths.len() + BATCH_SIZE - 1) / BATCH_SIZE; // Calculate total number of batches
    for (current_batch, batch) in all_paths.chunks(BATCH_SIZE).enumerate() {
        println!("Processing batch {}/{}", current_batch + 1, total_batches); // Show the current batch being processed
        let batch: Vec<PathBuf> = batch.to_vec();
        processor(batch);
        SHOULD_RESET.store(true, Ordering::SeqCst);
    }
}

fn processor(list_of_sync_files: Vec<PathBuf>) {
    let deduplicated_file_list = executor::filter::filter(list_of_sync_files);
    importer::import(&deduplicated_file_list.clone()).unwrap();
    let database = executor::databaser::databaser(deduplicated_file_list);
    executor::compressor::compressor(database);
}

pub fn prepare_progress_bar(len: u64) -> ProgressBar {
    let progress_bar = ProgressBar::new(len);
    progress_bar.set_draw_target(ProgressDrawTarget::stderr());
    let style = ProgressStyle::default_bar()
        .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} {msg}")
        .unwrap()
        .progress_chars("##-");
    progress_bar.set_style(style);
    progress_bar
}
