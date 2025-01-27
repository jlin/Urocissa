use self::processor::{process_image_info, process_video_info};
use crate::public::constant::VALID_IMAGE_EXTENSIONS;
use crate::public::database_struct::database::definition::Database;
use crate::public::error_data::{handle_error, ErrorData};
use crate::public::redb::DATA_TABLE;
use crate::public::tree::TREE;
use crate::synchronizer::video::VIDEO_QUEUE_SENDER;
use arrayvec::ArrayString;
use dashmap::DashMap;
use dashmap::DashSet;
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use std::cmp;
use std::panic::Location;
use std::sync::Arc;
pub mod fix_orientation;
pub mod generate_compressed_video;
pub mod generate_dynamic_image;
pub mod generate_exif;
pub mod generate_image_hash;
pub mod generate_thumbnail;
pub mod generate_width_height;
pub mod processor;
pub mod video_ffprobe;

pub fn databaser(vec_of_hash_alias: DashMap<ArrayString<64>, Database>) -> usize {
    let progress_bar = ProgressBar::new(vec_of_hash_alias.len() as u64);
    progress_bar.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} ({eta}) {msg}")
            .unwrap() // Added {msg} to the template
            .progress_chars("##-"),
    );

    progress_bar.set_message("Indexing...");

    let progress_bar = Arc::new(progress_bar);

    let write_txn = TREE.in_disk.begin_write().unwrap();
    let video_hash_dashset = DashSet::new();
    let successfully_handled_length = {
        let mut write_table = write_txn.open_table(DATA_TABLE).unwrap();

        let vec: Vec<_> = vec_of_hash_alias
            .into_par_iter()
            .filter_map(|(_, mut database)| {
                if VALID_IMAGE_EXTENSIONS.contains(&database.ext.as_str()) {
                    match process_image_info(&mut database) {
                        Ok(_) => {
                            // Update the progress bar
                            progress_bar.inc(1);
                            Some(database)
                        }
                        Err(e) => {
                            handle_error(ErrorData::new(
                                e.to_string(),
                                format!("An error occurred while processing file",),
                                Some(database.hash),
                                Some(database.source_path()),
                                Location::caller(),
                                Some(database),
                            ));
                            None
                        }
                    }
                } else {
                    match process_video_info(&mut database) {
                        Ok(_) => {
                            progress_bar.inc(1);
                            video_hash_dashset.insert(database.hash);
                            database.pending = true; // Waiting to perform the next step (generate_compressed) in a worker thread
                            Some(database)
                        }
                        Err(e) => {
                            handle_error(ErrorData::new(
                                e.to_string(),
                                format!("An error occurred while processing file",),
                                Some(database.hash),
                                Some(database.source_path()),
                                Location::caller(),
                                Some(database),
                            ));
                            None
                        }
                    }
                }
            })
            .collect();

        progress_bar.finish_with_message(format!("Index completed"));
        vec.iter().for_each(|database| {
            write_table.insert(&*database.hash, database).unwrap();
        });
        vec.len()
    };
    write_txn.commit().unwrap();
    // Send video hashes to the worker thread
    VIDEO_QUEUE_SENDER
        .get()
        .unwrap()
        .send(video_hash_dashset.into_iter().collect())
        .unwrap();
    successfully_handled_length
}

pub fn small_width_height(width: u32, height: u32, small_height: u32) -> (u32, u32) {
    let (nwidth, nheight) = if width >= cmp::max(height, small_height) {
        (small_height, height * small_height / width)
    } else if height >= cmp::max(width, small_height) {
        (width * small_height / height, small_height)
    } else {
        (width, height)
    };
    return (nwidth, nheight);
}
