use self::image_compressor::image_compressor;
use self::video_compressor::video_compressor;
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
pub mod image_compressor;
pub mod image_decoder;
mod image_thumbhash;
mod utils;
pub mod video_compressor;
mod video_ffprobe;
mod video_preview;
pub fn compressor<T>(databases: T)
where
    T: ParallelIterator<Item = DataBase>,
{
    let processed_count = Arc::new(AtomicUsize::new(0));
    let progress_bar = ProgressBar::new_spinner();
    progress_bar.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} {pos} files processed")
            .expect("Failed to set progress bar template"),
    );

    let video_hash_dashset = DashSet::new();
    let collect: Vec<DataBase> = databases
        .filter_map(|mut database| {
            let compress_result = if VALID_IMAGE_EXTENSIONS.contains(&database.ext.as_str()) {
                image_compressor
            } else {
                video_hash_dashset.insert(database.hash);
                video_compressor
            }(&mut database);
            processed_count.fetch_add(1, Ordering::SeqCst);
            progress_bar.inc(1);
            if let Err(error) = compress_result {
                handle_error(ErrorData::new(
                    error.to_string(),
                    format!("An error occurred while processing file",),
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
    let write_txn = TREE.in_disk.begin_write().unwrap();
    {
        let mut write_table = write_txn.open_table(DATA_TABLE).unwrap();
        collect.into_iter().for_each(|database| {
            write_table.insert(&*database.hash, &database).unwrap();
        });
    }
    write_txn.commit().unwrap();

    // Send video hashes to worker thread
    VIDEO_QUEUE_SENDER
        .get()
        .unwrap()
        .send(video_hash_dashset.into_iter().collect())
        .unwrap();

    progress_bar.finish();
}
