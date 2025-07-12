mod validator_extension;
mod validator_hash;
use crate::structure::database_struct::database::definition::Database;
use std::path::Path;

pub fn filter(path: &Path) -> anyhow::Result<Database> {
    let mut database = Database::new(path)?;
    validator_hash::validator(&mut database)?;
    Ok(database)
}
