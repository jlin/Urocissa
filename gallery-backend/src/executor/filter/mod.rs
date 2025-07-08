mod validator_extension;
mod validator_hash;
mod validator_modified;
use crate::structure::database_struct::database::definition::Database;
use anyhow::Result;
use std::path::PathBuf;

pub fn filter(path: PathBuf) -> Result<Database> {
    validator_extension::validator(&path)?;
    let database = validator_modified::validator(path)?;
    let vec_of_file_modify_hash = validator_hash::validator(database);
    vec_of_file_modify_hash
}
