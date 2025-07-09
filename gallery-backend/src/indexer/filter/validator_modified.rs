use crate::structure::database_struct::database::definition::Database;
use crate::structure::database_struct::file_modify::FileModify;
use anyhow::{Context, Result};
use path_clean::PathClean;
use std::{fs::metadata, path::PathBuf, time::UNIX_EPOCH};

pub fn validator(path: PathBuf) -> Result<Database> {
    let metadata = metadata(&path)
        .with_context(|| format!("[validator] Failed to read metadata for {}", path.display()))?;

    let modified = metadata.modified().with_context(|| {
        format!(
            "[validator] Failed to get modification time for {}",
            path.display()
        )
    })?;

    let modified_millis = modified
        .duration_since(UNIX_EPOCH)
        .with_context(|| {
            format!(
                "[validator] Modification time for {} is before UNIX_EPOCH",
                path.display()
            )
        })?
        .as_millis();

    let file_modify = FileModify::new(path.clean(), modified_millis);
    let size = metadata.len();
    let database = Database::new(size, file_modify);
    Ok(database)
}
