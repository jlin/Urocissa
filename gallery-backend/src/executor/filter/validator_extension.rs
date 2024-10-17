use crate::public::{
    constant::{VALID_IMAGE_EXTENSIONS, VALID_VIDEO_EXTENSIONS},
    error_data::{handle_error, ErrorData},
};
use rayon::prelude::*;
use std::{ffi::OsStr, panic::Location, path::PathBuf};
pub fn validator(all_paths: Vec<PathBuf>) -> impl ParallelIterator<Item = PathBuf> {
    all_paths.into_par_iter().filter_map(move |file_path| {
        let extension = file_path.extension().and_then(OsStr::to_str);
        match extension {
            Some(ext) => {
                let lowercased_ext = ext.to_ascii_lowercase();
                if VALID_IMAGE_EXTENSIONS.contains(&lowercased_ext.as_str())
                    || VALID_VIDEO_EXTENSIONS.contains(&lowercased_ext.as_str())
                {
                    Some(file_path)
                } else {
                    println!("{} is not a valid extension", &lowercased_ext);
                    None
                }
            }
            None => {
                handle_error(ErrorData::new(
                    format!("Could not determine the file extension"),
                    format!("Error occur when processing validator"),
                    None,
                    Some(file_path),
                    Location::caller(),
                ));
                None
            }
        }
    })
}
