// album.rs
use crate::constant::redb::{ALBUM_TABLE, DATA_TABLE};
use crate::db::tree::TREE;
use crate::structure::abstract_data::AbstractData;

use anyhow::Context;
use arrayvec::ArrayString;
use log::info;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use redb::ReadableTable;

#[derive(Debug)]
pub struct AlbumTask {
    pub album_id: ArrayString<64>,
}

impl AlbumTask {
    pub fn new(album_id: ArrayString<64>) -> Self {
        Self { album_id }
    }
}

pub fn album_task(task: AlbumTask) -> anyhow::Result<()> {
    info!("Perform album self-update");

    let txn = TREE
        .in_disk
        .begin_write()
        .context("begin_write failed (album)")?;
    {
        let mut album_table = txn.open_table(ALBUM_TABLE)?;
        let album_id = task.album_id;

        let album_opt = album_table
            .get(&*album_id)
            .unwrap()
            .map(|guard| guard.value());

        match album_opt {
            Some(mut album) => {
                album.pending = true;
                album.self_update();
                album.pending = false;
                album_table.insert(&*album_id, album).unwrap();
            }
            _ => {
                // Album has been deleted
                let ref_data = TREE.in_memory.read().unwrap();

                // Collect all data contained in this album
                let hash_list: Vec<_> = ref_data
                    .par_iter()
                    .filter_map(|dt| match &dt.abstract_data {
                        AbstractData::Database(db) if db.album.contains(&*album_id) => {
                            Some(db.hash)
                        }
                        _ => None,
                    })
                    .collect();

                let mut table = txn.open_table(DATA_TABLE).unwrap();

                // Remove this album from these data
                hash_list.into_iter().for_each(|hash| {
                    let mut database = table.get(&*hash).unwrap().unwrap().value();
                    database.album.remove(&*album_id);
                    table.insert(&*hash, database).unwrap();
                });
            }
        }
    }
    txn.commit().context("commit failed (album)")?;
    Ok(())
}
