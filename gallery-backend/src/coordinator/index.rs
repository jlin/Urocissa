use crate::{indexer::indexer, structure::database_struct::database::definition::Database};

pub fn index_task(database: Database) -> anyhow::Result<()> {
    indexer(database)?;
    Ok(())
}
