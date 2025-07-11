use crate::structure::database_struct::database::definition::Database;
use crate::structure::database_struct::file_modify::FileModify;
use anyhow::Context;
use path_clean::PathClean;
use std::{fs::metadata, path::PathBuf, time::UNIX_EPOCH};

pub fn validator(path: &PathBuf) -> anyhow::Result<Database> {
    let metadata = metadata(path).context(format!("Failed to read metadata: {:?}", path))?;

    let modified = metadata
        .modified()
        .context(format!("Failed to get modification time: {:?}", path))?;

    let modified_millis = modified
        .duration_since(UNIX_EPOCH)
        .context(format!(
            "Modification time is before UNIX_EPOCH: {:?}",
            path
        ))?
        .as_millis();

    let file_modify = FileModify::new(path.clean(), modified_millis);
    let size = metadata.len();
    let database = Database::new(size, file_modify);
    Ok(database)
}
