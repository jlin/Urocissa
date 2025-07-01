use crate::constant::{VALID_IMAGE_EXTENSIONS, VALID_VIDEO_EXTENSIONS};
use crate::public::error_data::{ErrorData, handle_error};
use crate::synchronizer::delete::delete_paths;
use rayon::iter::Either;
use rayon::prelude::*;
use std::{ffi::OsStr, panic::Location, path::PathBuf};

pub fn validator(all_paths: Vec<PathBuf>) -> impl ParallelIterator<Item = PathBuf> {
    let (valids, invalids): (Vec<PathBuf>, Vec<PathBuf>) =
        all_paths.into_par_iter().partition_map(|file_path| {
            let ext_opt = file_path.extension().and_then(OsStr::to_str);
            match ext_opt {
                Some(ext) => {
                    let lower = ext.to_ascii_lowercase();
                    if VALID_IMAGE_EXTENSIONS.contains(&lower.as_str())
                        || VALID_VIDEO_EXTENSIONS.contains(&lower.as_str())
                    {
                        Either::Left(file_path)
                    } else {
                        error!("{} is not a valid extension", &lower);
                        Either::Right(file_path)
                    }
                }
                None => {
                    handle_error(ErrorData::new(
                        "Could not determine the file extension".to_string(),
                        "Error occur when processing validator".to_string(),
                        None,
                        Some(file_path.clone()),
                        Location::caller(),
                        None,
                    ));
                    Either::Right(file_path)
                }
            }
        });

    if !invalids.is_empty() {
        delete_paths(invalids);
    }

    valids.into_par_iter()
}
