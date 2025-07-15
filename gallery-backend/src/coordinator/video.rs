use anyhow::Context;

use crate::{
    batcher::flush_tree::FLUSH_TREE_QUEUE,
    indexer::databaser::generate_compressed_video::generate_compressed_video,
    looper::{LOOPER, Signal},
    structure::database_struct::database::definition::Database,
    tui::DASHBOARD,
};

pub fn video_task(mut database: Database) -> anyhow::Result<()> {
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
