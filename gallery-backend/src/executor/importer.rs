use std::path::PathBuf;
use std::sync::Arc;
use std::time::Instant; // Import Instant for time measurement

use crate::public::database_struct::database::definition::DataBase;
use arrayvec::ArrayString;
use dashmap::DashMap;
use futures::stream::{self, StreamExt};
use futures::TryStreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use log::error;
use tokio::fs;
use tokio::io;

/// Synchronous import function that internally uses Tokio's async capabilities
pub fn import(
    deduplicated_file_list: &DashMap<ArrayString<64>, DataBase>,
) -> Result<(), Box<dyn std::error::Error>> {
    // Start time measurement
    let start_time = Instant::now();

    // Initialize the progress bar
    let progress_bar = ProgressBar::new(deduplicated_file_list.len() as u64);
    progress_bar.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} ({eta}) {msg}")? // Added {msg} to the template
            .progress_chars("##-"),
    );

    // Set an initial empty message
    progress_bar.set_message("");

    // Wrap the progress bar in an Arc to allow sharing across threads/tasks
    let progress_bar = Arc::new(progress_bar);

    // Create a new Tokio runtime
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?;

    // Execute the asynchronous import logic within the runtime
    if let Err(e) = rt.block_on(async_import(deduplicated_file_list, progress_bar.clone())) {
        // Calculate elapsed time in case of error
        let elapsed = start_time.elapsed();
        // Set error message
        progress_bar.set_message(format!("Error encountered after {:?}", elapsed));
        // Finish the progress bar to display the message
        progress_bar.finish_with_message(format!("Import failed in {:?}", elapsed));
        return Err(e);
    }

    // Calculate elapsed time
    let elapsed = start_time.elapsed();

    // Set completion message
    progress_bar.set_message(format!("Import completed in {:?}", elapsed));

    // Finalize the progress bar with the completion message
    progress_bar.finish_with_message(format!("Import completed in {:?}", elapsed));

    Ok(())
}

/// Asynchronous helper function to perform concurrent file copying
async fn async_import(
    deduplicated_file_list: &DashMap<ArrayString<64>, DataBase>,
    progress_bar: Arc<ProgressBar>,
) -> Result<(), Box<dyn std::error::Error>> {
    // Define the level of concurrency (number of simultaneous operations)
    let concurrency_limit = 10;

    // Create a stream of asynchronous tasks for each file to be copied
    let copy_tasks = stream::iter(deduplicated_file_list.iter())
        .map(|entry| {
            let database = entry.value();
            // Clone the Arc to share the progress bar across tasks
            let source_path = database.source_path();
            let dest_path = database.imported_path();
            let progress_bar = Arc::clone(&progress_bar);
            async move {
                // Ensure the destination directory exists
                if let Some(parent) = dest_path.parent() {
                    if let Err(err) = fs::create_dir_all(parent).await {
                        error!("Failed to create directory {:?}: {:?}", parent, err);
                        return Err(err);
                    }
                }

                // Perform the file copy
                if let Err(err) = fs::copy(&source_path, &dest_path).await {
                    error!(
                        "Failed to copy file from {:?} to {:?}: {:?}",
                        source_path, dest_path, err
                    );
                    return Err(err);
                }

                // Update the progress bar
                progress_bar.inc(1);
                Ok::<(), io::Error>(())
            }
        })
        .buffer_unordered(concurrency_limit); // Limit the number of concurrent tasks

    // Execute all copy tasks concurrently, stopping on the first error
    copy_tasks.try_for_each(|_| async { Ok(()) }).await?;

    Ok(())
}
