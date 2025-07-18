use crate::public::constant::redb::{ALBUM_TABLE, DATA_TABLE};
use crate::public::db::tree::TREE;
use crate::public::error_data::handle_error;
use crate::public::structure::abstract_data::AbstractData;
use anyhow::Context;
use anyhow::Result;
use arrayvec::ArrayString;
use log::info;
use mini_executor::Task;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use redb::ReadableTable;
use tokio::task::spawn_blocking;

pub struct AlbumTask {
    album_id: ArrayString<64>,
}

impl AlbumTask {
    pub fn new(album_id: ArrayString<64>) -> Self {
        Self { album_id }
    }
}

impl Task for AlbumTask {
    type Output = Result<()>;

    fn run(self) -> impl std::future::Future<Output = Self::Output> + Send {
        async move {
            spawn_blocking(move || album_task(self.album_id))
                .await
                .expect("blocking task panicked")
                .map_err(|err| handle_error(err.context("Failed to run album task")))
        }
    }
}

pub fn album_task(album_id: ArrayString<64>) -> Result<()> {
    info!("Perform album self-update");

    let txn = TREE
        .in_disk
        .begin_write()
        .context("begin_write failed (album)")?;
    {
        let mut album_table = txn.open_table(ALBUM_TABLE)?;

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
