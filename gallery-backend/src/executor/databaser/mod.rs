use std::panic::Location;

use self::processor_image::process_image_info;
use self::processor_video::process_video_info;
use crate::public::constant::VALID_IMAGE_EXTENSIONS;
use crate::public::database_struct::database::definition::DataBase;
use crate::public::database_struct::hash_alias::HashAliasSize;
use crate::public::error_data::{handle_error, ErrorData};
use rayon::prelude::*;
mod processor_image;
mod processor_video;
pub fn databaser(vec_of_hash_alias: Vec<HashAliasSize>) -> impl ParallelIterator<Item = DataBase> {
    vec_of_hash_alias
        .into_par_iter()
        .filter_map(move |mut hash_alias_size| {
            if VALID_IMAGE_EXTENSIONS.contains(&hash_alias_size.hash_alias.ext().as_str()) {
                let database = process_image_info(hash_alias_size);
                return Some(database);
            } else {
                match process_video_info(&mut hash_alias_size) {
                    Ok(database) => Some(database),
                    Err(e) => {
                        handle_error(ErrorData::new(
                            e.to_string(),
                            format!("An error occurred while processing file",),
                            Some(hash_alias_size.hash_alias.hash),
                            Some(hash_alias_size.hash_alias.source_path().clone()),
                            Location::caller(),
                        ));
                        None
                    }
                }
            }
        })
}
