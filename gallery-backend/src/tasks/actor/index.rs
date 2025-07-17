use anyhow::Context;
use anyhow::Result;
use anyhow::anyhow;
use tokio_rayon::spawn;

use crate::{
    process::info::{process_image_info, process_video_info},
    public::{
        constant::VALID_IMAGE_EXTENSIONS,
        error_data::handle_error,
        structure::{database_struct::database::definition::Database, guard::PendingGuard},
        tui::{DASHBOARD, FileType},
    },
    tasks::{COORDINATOR, batcher::flush_tree::FlushTreeTask},
};
use mini_coordinator::Task;
pub struct IndexTask {
    pub database: Database,
}

impl IndexTask {
    pub fn new(database: Database) -> Self {
        Self { database }
    }
}

impl Task for IndexTask {
    type Output = Result<Database>;

    fn run(self) -> impl std::future::Future<Output = Self::Output> + Send {
        async move {
            let _pending_guard = PendingGuard::new();
            spawn(move || index_task(self.database))
                .await
                .map_err(|err| handle_error(err.context("Failed to run index task")))
        }
    }
}

fn index_task(mut database: Database) -> Result<Database> {
    let hash = database.hash;
    let newest_path = database
        .alias
        .iter()
        .max()
        .ok_or_else(|| anyhow!("alias collection is empty for hash: {}", hash))?
        .file
        .clone();

    // Register task in dashboard; attach context if extension is invalid
    DASHBOARD.add_task(
        hash,
        newest_path.clone(),
        FileType::try_from(database.ext_type.as_str())
            .context(format!("unsupported file type: {}", database.ext_type))?,
    );

    // Branch processing based on file extension
    let is_image = VALID_IMAGE_EXTENSIONS.contains(&database.ext.as_str());
    if is_image {
        process_image_info(&mut database).context("failed to process image metadata pipeline")?;
    } else {
        process_video_info(&mut database).context("failed to process video metadata pipeline")?;
        database.pending = true;
    }

    // Persist the updated record & advance progress state
    COORDINATOR.execute_batch_detached(FlushTreeTask::new(vec![database.clone()]));
    DASHBOARD.advance_task_state(&hash);

    Ok(database)
}
