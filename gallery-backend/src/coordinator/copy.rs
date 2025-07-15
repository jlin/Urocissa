use std::fs;

use anyhow::Context;

use crate::{
    coordinator::{COORDINATOR, Task},
    structure::database_struct::database::definition::Database,
};

pub fn copy_task(database: Database) -> anyhow::Result<()> {
    let source_path = database.source_path();
    let dest_path = database.imported_path();

    if let Some(parent) = dest_path.parent() {
        fs::create_dir_all(parent).context(format!("failed to create directory: {parent:?}"))?;
    }

    fs::copy(&source_path, &dest_path)
        .context(format!("failed to copy {source_path:?} → {dest_path:?}"))?;
    COORDINATOR.submit(Task::Index(database))?;
    Ok(())
}
