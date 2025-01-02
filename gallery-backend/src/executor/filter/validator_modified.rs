use crate::public::database_struct::database::definition::DataBase;
use crate::public::database_struct::file_modify::FileModify;
use crate::public::error_data::{handle_error, ErrorData};
use path_clean::PathClean;
use rayon::prelude::*;
use std::{fs::metadata, panic::Location, path::PathBuf, time::UNIX_EPOCH};

pub fn validator<I>(file_paths: I) -> impl ParallelIterator<Item = DataBase>
where
    I: ParallelIterator<Item = PathBuf>,
{
    file_paths.filter_map(|file_path| {
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
                        let database = DataBase::new(size, file_modify);
                        Some(database)
                    }
                    Err(err) => {
                        handle_error(ErrorData::new(
                            err.to_string(),
                            "An error occurred while getting the file modified time".to_string(),
                            None,
                            Some(file_path),
                            Location::caller(),
                            None,
                        ));
                        None
                    }
                }
            }
            Err(err) => {
                handle_error(ErrorData::new(
                    err.to_string(),
                    "An error occurred while getting the file metadata".to_string(),
                    None,
                    Some(file_path),
                    Location::caller(),
                    None,
                ));
                None
            }
        }
    })
}
