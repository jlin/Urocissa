use std::panic::Location;

use self::processor_image::process_image_info;
use self::processor_video::process_video_info;
use crate::public::constant::VALID_IMAGE_EXTENSIONS;
use crate::public::database_struct::database::definition::DataBase;
use crate::public::error_data::{handle_error, ErrorData};
use arrayvec::ArrayString;
use dashmap::DashMap;
use rayon::prelude::*;
mod generate_dynamic_image;
mod generate_exif;
mod generate_width_height;
mod processor_image;
mod processor_video;
pub fn databaser(
    vec_of_hash_alias: DashMap<ArrayString<64>, DataBase>,
) -> impl ParallelIterator<Item = DataBase> {
    vec_of_hash_alias
        .into_par_iter()
        .filter_map(move |(hash, mut database)| {
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
            }
        })
}
