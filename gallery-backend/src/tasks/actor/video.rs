use crate::{
    operations::indexation::generate_compressed_video::generate_compressed_video,
    public::{
        structure::{database_struct::database::definition::Database, guard::PendingGuard},
        tui::DASHBOARD,
    },
    tasks::batcher::flush_tree::FLUSH_TREE_QUEUE,
};
use anyhow::Context;
use anyhow::Result;
use mini_actor::Task;
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
            let result = spawn(move || video_task(self.database))
                .await
                .expect("blocking task panicked");
            Ok(result)
        }
    }
}

pub fn video_task(mut database: Database) -> Result<()> {
    let hash = database.hash;
    match generate_compressed_video(&mut database) {
        Ok(_) => {
            database.pending = false;
            FLUSH_TREE_QUEUE.update(vec![database]);
            DASHBOARD.advance_task_state(&hash);
        }
        Err(err) => Err(err).context(format!(
            "video_task: video compression failed for hash: {}",
            hash
        ))?,
    }
    Ok(())
}
