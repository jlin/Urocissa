use crate::{indexer::indexer, structure::database_struct::database::definition::Database};

#[derive(Debug)]
pub struct IndexTask {
    pub database: Database,
}
impl IndexTask {
    pub fn new(database: Database) -> Self {
        Self { database }
    }
}

pub fn index_task(task: IndexTask) -> anyhow::Result<()> {
    indexer(task.database)?;
    Ok(())
}
