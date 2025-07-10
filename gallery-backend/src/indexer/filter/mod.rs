mod validator_extension;
mod validator_hash;
mod validator_modified;
use crate::structure::database_struct::database::definition::Database;
use std::path::PathBuf;

pub fn filter(path: PathBuf) -> anyhow::Result<Database> {
    validator_extension::validator(&path)?;
    let mut database = validator_modified::validator(path)?;
    validator_hash::validator(&mut database)?;
    Ok(database)
}
