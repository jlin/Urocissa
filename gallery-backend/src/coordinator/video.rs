use core::hash;

use anyhow::Context;
use arrayvec::ArrayString;
use log::info;

use crate::{
    batcher::flush_tree::FLUSH_TREE_QUEUE,
    db::tree::TREE,
    indexer::databaser::generate_compressed_video::generate_compressed_video,
    looper::{LOOPER, Signal},
    structure::database_struct::database::definition::Database,
    tui::DASHBOARD,
};

#[derive(Debug)]
pub struct VideoTask {
    pub database: Database,
}

impl VideoTask {
    pub fn new(database: Database) -> Self {
        Self { database }
    }
}

pub fn video_task(task: VideoTask) -> anyhow::Result<()> {
    let mut database = task.database;
    let hash = database.hash;
    match generate_compressed_video(&mut database) {
        Ok(_) => {
            database.pending = false;

            FLUSH_TREE_QUEUE.update(vec![database]);
            LOOPER.notify(Signal::UpdateTree);

            DASHBOARD.advance_task_state(&hash);
            Ok(())
        }
        Err(err) => Err(err).context(format!(
            "video_task: video compression failed for hash: {}",
            hash
        )),
    }
}
