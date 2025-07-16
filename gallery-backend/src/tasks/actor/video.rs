use anyhow::Context;
use tokio_rayon::spawn;

use crate::{
    operations::indexation::generate_compressed_video::generate_compressed_video,
    public::{structure::database_struct::database::definition::Database, tui::DASHBOARD},
    tasks::{
        batcher::flush_tree::FLUSH_TREE_QUEUE,
        looper::{LOOPER, Signal},
    },
};
use mini_actor::Task;
pub struct VideoTask {
    database: Database,
}

impl VideoTask {
    pub fn new(database: Database) -> Self {
        Self { database }
    }
}

impl Task for VideoTask {
    type Output = anyhow::Result<()>;

    fn run(self) -> impl std::future::Future<Output = Self::Output> + Send {
        async move {
            let result = spawn(move || video_task(self.database))
                .await
                .expect("blocking task panicked");
            Ok(result)
        }
    }
}

pub fn video_task(mut database: Database) -> anyhow::Result<()> {
    DASHBOARD.increase_pending();
    let hash = database.hash;
    match generate_compressed_video(&mut database) {
        Ok(_) => {
            database.pending = false;

            FLUSH_TREE_QUEUE.update(vec![database]);
            LOOPER.notify(Signal::UpdateTree);

            DASHBOARD.advance_task_state(&hash);
        }
        Err(err) => Err(err).context(format!(
            "video_task: video compression failed for hash: {}",
            hash
        ))?,
    }
    DASHBOARD.decrease_pending();
    Ok(())
}
