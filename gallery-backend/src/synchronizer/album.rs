use crate::public::abstract_data::AbstractData;
use crate::public::redb::{ALBUM_TABLE, DATA_TABLE};
use crate::public::tree::start_loop::SHOULD_RESET;
use crate::public::tree::TREE;

use arrayvec::ArrayString;
use log::info;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use redb::ReadableTable;
use std::collections::HashSet;
use std::sync::OnceLock;
use tokio;
use tokio::sync::mpsc::{unbounded_channel, UnboundedSender};

pub static ALBUM_SELFUPDATE_QUEUE_SENDER: OnceLock<UnboundedSender<Vec<ArrayString<64>>>> =
    OnceLock::new();
pub fn start_album_channel() -> tokio::task::JoinHandle<()> {
    let (album_selfupdate_queue_sender, mut album_selfupdate_queue_receiver) =
        unbounded_channel::<Vec<ArrayString<64>>>();
    ALBUM_SELFUPDATE_QUEUE_SENDER
        .set(album_selfupdate_queue_sender)
        .unwrap();

    tokio::task::spawn(async move {
        while let Some(list_of_album_id) = album_selfupdate_queue_receiver.recv().await {
            tokio::task::spawn_blocking(move || {
                // Deduplicate
                let unique_id: HashSet<_> = list_of_album_id.into_iter().collect();
                let id_vec: Vec<_> = unique_id.into_iter().collect();
                let txn = TREE.in_disk.begin_write().unwrap();
                {
                    let mut album_table = txn.open_table(ALBUM_TABLE).unwrap();
                    id_vec.into_iter().for_each(|album_id| {
                        let album_opt = album_table
                            .get(&*album_id)
                            .unwrap()
                            .map(|guard| guard.value());

                        if let Some(mut album) = album_opt {
                            album.pending = true;
                            album.self_update();
                            album.pending = false;
                            album_table.insert(&*album_id, album).unwrap();
                        } else {
                            // Album has been deleted
                            let ref_data = TREE.in_memory.read().unwrap();

                            // Collect all data contained in this album
                            let hash_list: Vec<_> = ref_data
                                .par_iter()
                                .filter_map(|dt| match &dt.abstract_data {
                                    AbstractData::DataBase(db) if db.album.contains(&album_id) => {
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
                    });
                }
                txn.commit().unwrap();
                SHOULD_RESET.notify_one();
                info!("Album self-updated")
            })
            .await
            .unwrap();
        }
    })
}
