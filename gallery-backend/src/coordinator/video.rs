use anyhow::Context;
use arrayvec::ArrayString;

use crate::{
    constant::redb::DATA_TABLE,
    db::tree::TREE,
    indexer::databaser::generate_compressed_video::generate_compressed_video,
    looper::{LOOPER, Signal},
};

#[derive(Debug)]
pub struct VideoTask {
    pub hash: ArrayString<64>,
}

impl VideoTask {
    pub fn new(hash: ArrayString<64>) -> Self {
        Self { hash }
    }
}

pub fn video_task(task: VideoTask) -> anyhow::Result<()> {
    let hash = task.hash;
    let read_table = TREE.api_read_tree();
    let guard = read_table.get(&*hash).unwrap();

    let mut database = if let Some(guard) = guard {
        guard.value()
    } else {
        anyhow::bail!("video_task: hash not found in database: {hash}");
    };

    match generate_compressed_video(&mut database) {
        Ok(_) => {
            database.pending = false;
            let write_txn = TREE.in_disk.begin_write().unwrap();
            {
                let mut write_table = write_txn.open_table(DATA_TABLE).unwrap();
                write_table.insert(&*database.hash, &database).unwrap();
            }
            write_txn.commit().unwrap();
            LOOPER.notify(Signal::UpdateTree);
            Ok(())
        }
        Err(err) => Err(err)
            .with_context(|| format!("video_task: video compression failed for hash: {}", hash)),
    }
}
