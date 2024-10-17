use crate::public::database_struct::file_modify::{FileModify, FileModifySize};
use crate::public::error_data::{handle_error, ErrorData};
use dashmap::DashSet;
use path_clean::PathClean;
use rayon::prelude::*;
use std::{fs::metadata, panic::Location, path::PathBuf, time::UNIX_EPOCH};
pub fn validator<I>(file_paths: I) -> DashSet<FileModifySize>
where
    I: ParallelIterator<Item = PathBuf>,
{
    let dash_set_of_file_modify = DashSet::new();
    file_paths.for_each(|file_path| {
        let metadata_result = metadata(&file_path);
        match metadata_result {
            Ok(metadata) => {
                let modified_result = metadata.modified();
                match modified_result {
                    Ok(modified) => {
                        let modified_millis =
                            modified.duration_since(UNIX_EPOCH).unwrap().as_millis();
                        let file_modify = FileModify::new(file_path.clean(), modified_millis);
                        let size = metadata.len();
                        let file_modify_size = FileModifySize::new(file_modify, size);
                        dash_set_of_file_modify.insert(file_modify_size);
                    }
                    Err(err) => {
                        handle_error(ErrorData::new(
                            err.to_string(),
                            format!("An error occurred while getting the file modified time",),
                            None,
                            Some(file_path),
                            Location::caller(),
                        ));
                    }
                }
            }
            Err(err) => {
                handle_error(ErrorData::new(
                    err.to_string(),
                    format!("An error occurred while getting the file modified time",),
                    None,
                    Some(file_path),
                    Location::caller(),
                ));
            }
        }
    });
    dash_set_of_file_modify
}
