use crate::public::error_data::{handle_error, ErrorData};
use crate::public::tree::TREE;
use crate::public::{
    constant::VALID_IMAGE_EXTENSIONS, database_struct::database::definition::DataBase,
    redb::DATA_TABLE,
};
use crate::synchronizer::video::VIDEO_QUEUE_SENDER;
use dashmap::DashSet;
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use std::panic::Location;
use std::sync::atomic::Ordering;
use std::sync::{atomic::AtomicUsize, Arc};

use super::image_compressor::image_compressor;
use super::video_compressor::video_compressor;
pub fn compressor<T>(databases: T)
where
    T: ParallelIterator<Item = DataBase>,
{
    // Initialize the processed count
    let processed_count = Arc::new(AtomicUsize::new(0));

    // Initialize the spinner progress bar
    let progress_bar = ProgressBar::new_spinner();

    // Define the active style with spinner and space
    let active_style = ProgressStyle::default_spinner()
        .template("[{elapsed_precise}] {spinner:.green} {pos} files processed")
        .expect("Failed to set active progress bar template");

    // Define the finished style without spinner and space
    let finished_style = ProgressStyle::default_spinner()
        .template("[{elapsed_precise}] {pos} files processed")
        .expect("Failed to set finished progress bar template");

    // Set the active style before starting processing
    progress_bar.set_style(active_style);

    // Initialize DashSet for video hashes
    let video_hash_dashset = DashSet::new();

    // Start processing the databases in parallel
    let collect: Vec<DataBase> = databases
        .filter_map(|mut database| {
            // Determine the appropriate compressor based on file extension
            let compress_result = if VALID_IMAGE_EXTENSIONS.contains(&database.ext.as_str()) {
                image_compressor(&mut database)
            } else {
                video_hash_dashset.insert(database.hash);
                video_compressor(&mut database)
            };

            // Increment the processed count atomically
            processed_count.fetch_add(1, Ordering::SeqCst);

            // Update the progress bar
            progress_bar.inc(1);

            // Handle any compression errors
            if let Err(error) = compress_result {
                handle_error(ErrorData::new(
                    error.to_string(),
                    "An error occurred while processing file".to_string(),
                    Some(database.hash),
                    Some(database.imported_path()),
                    Location::caller(),
                    Some(database),
                ));
                None
            } else {
                Some(database)
            }
        })
        .collect();

    // Begin a write transaction
    let write_txn = TREE.in_disk.begin_write().unwrap();
    {
        let mut write_table = write_txn.open_table(DATA_TABLE).unwrap();
        collect.into_iter().for_each(|database| {
            write_table.insert(&*database.hash, &database).unwrap();
        });
    }
    write_txn.commit().unwrap();

    // Send video hashes to the worker thread
    VIDEO_QUEUE_SENDER
        .get()
        .unwrap()
        .send(video_hash_dashset.into_iter().collect())
        .unwrap();

    // Set the finished style after processing is complete
    progress_bar.set_style(finished_style);

    // Finalize the progress bar with the finished style
    progress_bar.finish_with_message(format!(
        "[{:?}] {} files processed",
        progress_bar.elapsed(),
        processed_count.load(Ordering::SeqCst)
    ));
}
