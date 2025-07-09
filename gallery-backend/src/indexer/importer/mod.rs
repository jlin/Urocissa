use crate::structure::database_struct::database::definition::Database;
use anyhow::Context;
use tokio::fs;
/// Synchronous import function that internally uses Tokio's async capabilities
pub fn import(database: &Database) -> anyhow::Result<()> {
    // Create a new Tokio runtime to perform async copy
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?;

    rt.block_on(async_import(database))?;

    Ok(())
}

/// Asynchronous helper function to perform concurrent file copying
async fn async_import(database: &Database) -> anyhow::Result<()> {
    let source_path = database.source_path();
    let dest_path = database.imported_path();

    if let Some(parent) = dest_path.parent() {
        fs::create_dir_all(parent)
            .await
            .with_context(|| format!("Failed to create directory: {:?}", parent))?;
    }

    fs::copy(&source_path, &dest_path)
        .await
        .with_context(|| format!("Failed to copy from {:?} to {:?}", source_path, dest_path))?;

    Ok(())
}
