use anyhow::Result;
use std::path::PathBuf;
use tokio_rayon::spawn;

use crate::{
    jobs::info::{process_image_info, process_video_info},
    public::{
        constant::VALID_IMAGE_EXTENSIONS,
        error_data::handle_error,
        structure::{database_struct::database::definition::Database, guard::PendingGuard},
        tui::{DASHBOARD, FileType},
    },
    tasks::{
        COORDINATOR,
        actor::{delete::DeleteTask, video::VideoTask},
        batcher::flush_tree::FLUSH_TREE_QUEUE,
    },
};
use mini_actor::Task;
pub struct IndexTask {
    pub database: Database,
}

impl IndexTask {
    pub fn new(database: Database) -> Self {
        Self { database }
    }
}

impl Task for IndexTask {
    type Output = ();

    fn run(self) -> impl std::future::Future<Output = Self::Output> + Send {
        async move {
            let _pending_guard = PendingGuard::new();
            match spawn(move || index_task(self.database)).await {
                Ok(_) => (),
                Err(err) => {
                    handle_error(err);
                }
            }
        }
    }
}

fn index_task(mut database: Database) -> Result<()> {
    let hash = database.hash;
    let newest_path = database.alias.iter().max().unwrap().file.clone();
    DASHBOARD.add_task(
        hash,
        newest_path.clone(),
        FileType::try_from(database.ext_type.as_str())?,
    );

    let is_image = VALID_IMAGE_EXTENSIONS.contains(&database.ext.as_str());
    if is_image {
        process_image_info(&mut database)?;
    } else {
        process_video_info(&mut database)?;
        database.pending = true;
        COORDINATOR.execute_detached(VideoTask::new(database.clone()));
    }

    COORDINATOR.execute_detached(DeleteTask::new(PathBuf::from(newest_path)));
    FLUSH_TREE_QUEUE.update(vec![database]);
    DASHBOARD.advance_task_state(&hash);

    Ok(())
}
