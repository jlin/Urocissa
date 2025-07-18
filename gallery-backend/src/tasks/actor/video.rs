use crate::{
    operations::indexation::generate_compressed_video::generate_compressed_video,
    public::{
        error_data::handle_error,
        structure::{database_struct::database::definition::Database, guard::PendingGuard},
        tui::DASHBOARD,
    },
    tasks::{COORDINATOR, batcher::flush_tree::FlushTreeTask},
};
use anyhow::Context;
use anyhow::Result;
use log::info;
use mini_coordinator::Task;
use tokio_rayon::spawn;

pub struct VideoTask {
    database: Database,
}

impl VideoTask {
    pub fn new(database: Database) -> Self {
        Self { database }
    }
}

impl Task for VideoTask {
    type Output = Result<()>;

    fn run(self) -> impl std::future::Future<Output = Self::Output> + Send {
        async move {
            let _pending_guard = PendingGuard::new();
            spawn(move || video_task(self.database))
                .await
                .map_err(|err| handle_error(err.context("Failed to run video task")))
        }
    }
}

pub fn video_task(mut database: Database) -> Result<()> {
    let hash = database.hash;
    info!("ready to generate compressed video for hash: {}", hash);
    match generate_compressed_video(&mut database) {
        Ok(_) => {
            info!("Video compression completed for hash: {}", hash);
            database.pending = false;
            COORDINATOR.execute_batch_detached(FlushTreeTask::new(vec![database]));
            info!("Database updated to vonersion done for hash: {}", hash);
            DASHBOARD.advance_task_state(&hash);
        }
        Err(err) => Err(err).context(format!(
            "video_task: video compression failed for hash: {}",
            hash
        ))?,
    }
    Ok(())
}
