use std::path::PathBuf;

use tokio_rayon::spawn;

use crate::{
    jobs::info::{process_image_info, process_video_info},
    public::{
        constant::VALID_IMAGE_EXTENSIONS,
        structure::{database_struct::database::definition::Database, guard::PendingGuard},
        tui::{DASHBOARD, FileType},
    },
    tasks::{
        COORDINATOR,
        actor::{delete::DeleteTask, video::VideoTask},
        batcher::{flush_tree::FLUSH_TREE_QUEUE, update_tree::UPDATE_TREE_QUEUE},
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
    type Output = anyhow::Result<()>;

    fn run(self) -> impl std::future::Future<Output = Self::Output> + Send {
        async move {
            let result = spawn(move || {
                let _pending_guard = PendingGuard::new();
                index_task(self.database)
            })
            .await?;
            Ok(result)
        }
    }
}

fn index_task(mut database: Database) -> anyhow::Result<()> {
    let hash = database.hash;
    let newest_path = database.alias.iter().max().unwrap().file.clone();
    DASHBOARD.add_task(
        hash,
        newest_path,
        FileType::try_from(database.ext_type.as_str())?,
    );

    let is_image = VALID_IMAGE_EXTENSIONS.contains(&database.ext.as_str());
    {
        if is_image {
            process_image_info(&mut database)?;
        } else {
            process_video_info(&mut database)?;

            database.pending = true;
        }

        if let Some(latest) = database.alias.iter().max_by_key(|a| a.scan_time) {
            COORDINATOR.execute_detached(DeleteTask::new(PathBuf::from(&latest.file)));
        };
        if !is_image {
            COORDINATOR.execute_detached(VideoTask::new(database.clone()));
        }
        FLUSH_TREE_QUEUE.update(vec![database]);
    }
    UPDATE_TREE_QUEUE.update(vec![()]);
    DASHBOARD.advance_task_state(&hash);

    Ok(())
}
