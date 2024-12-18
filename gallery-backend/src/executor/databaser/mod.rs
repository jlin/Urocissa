use std::cmp;
use std::panic::Location;

use self::processor::{process_image_info, process_video_info};
use crate::public::constant::VALID_IMAGE_EXTENSIONS;
use crate::public::database_struct::database::definition::DataBase;
use crate::public::error_data::{handle_error, ErrorData};
use crate::public::redb::DATA_TABLE;
use crate::public::tree::TREE;
use crate::synchronizer::video::VIDEO_QUEUE_SENDER;
use arrayvec::ArrayString;
use dashmap::DashMap;
use dashmap::DashSet;
use rayon::prelude::*;
pub mod fix_orientation;
pub mod generate_compressed_image;
pub mod generate_compressed_video;
pub mod generate_dynamic_image;
pub mod generate_exif;
pub mod generate_image_hash;
pub mod generate_preview;
pub mod generate_width_height;
pub mod image_decoder;
pub mod processor;
pub mod video_ffprobe;
pub fn databaser(vec_of_hash_alias: DashMap<ArrayString<64>, DataBase>) -> () {
    let write_txn = TREE.in_disk.begin_write().unwrap();
    let video_hash_dashset = DashSet::new();
    {
        let mut write_table = write_txn.open_table(DATA_TABLE).unwrap();

        let vec: Vec<_> = vec_of_hash_alias
            .into_par_iter()
            .filter_map(|(hash, mut database)| {
                if VALID_IMAGE_EXTENSIONS.contains(&database.ext.as_str()) {
                    match process_image_info(&mut database) {
                        Ok(_) => Some(database),
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
        vec.iter().for_each(|database| {
            write_table.insert(&*database.hash, database).unwrap();
        });
    }
    write_txn.commit().unwrap();
    // Send video hashes to the worker thread
    VIDEO_QUEUE_SENDER
        .get()
        .unwrap()
        .send(video_hash_dashset.into_iter().collect())
        .unwrap();
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
