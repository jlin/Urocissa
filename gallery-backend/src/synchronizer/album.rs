use crate::public::abstract_data::AbstractData;
use crate::public::redb::{ALBUM_TABLE, DATA_TABLE};
use crate::public::tree::start_loop::SHOULD_RESET;
use crate::public::tree::TREE;

use arrayvec::ArrayString;
use log::info;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use redb::ReadableTable;
use std::collections::HashSet;
use std::sync::atomic::Ordering;
use std::sync::OnceLock;
use tokio;
use tokio::sync::mpsc::{unbounded_channel, UnboundedSender};

pub static ALBUM_QUEUE_SENDER: OnceLock<UnboundedSender<Vec<ArrayString<64>>>> = OnceLock::new();
pub fn start_album_channel() {
    let (album_queue_sender, mut album_queue_receiver) =
        unbounded_channel::<Vec<ArrayString<64>>>();
    ALBUM_QUEUE_SENDER.set(album_queue_sender).unwrap();

    tokio::task::spawn(async move {
        while let Some(list_of_album_id) = album_queue_receiver.recv().await {
            tokio::task::spawn_blocking(move || {
                std::thread::sleep(std::time::Duration::from_millis(500));

                // Deduplication operation
                let unique_id: HashSet<_> = list_of_album_id.into_iter().collect();
                let id_vec: Vec<_> = unique_id.into_iter().collect();

                if !id_vec.is_empty() {
                    let txn = TREE.in_disk.begin_write().unwrap();
                    {
                        let mut album_table = txn.open_table(ALBUM_TABLE).unwrap();
                        id_vec.into_iter().for_each(|album_id| {
                            let album_opt = match album_table.get(&*album_id).unwrap() {
                                Some(album) => {
                                    let mut album = album.value();
                                    album.pending = true;
                                    album.self_update();
                                    album.pending = false;
                                    Some(album)
                                }
                                None => {
                                    let ref_data = TREE.in_memory.read().unwrap();

                                    let hash_list: Vec<_> = ref_data
                                        .par_iter()
                                        .filter_map(|database_timestamp| match &database_timestamp
                                            .abstract_data
                                        {
                                            AbstractData::DataBase(database) => {
                                                if database.album.contains(&album_id) {
                                                    Some(database.hash)
                                                } else {
                                                    None
                                                }
                                            }
                                            AbstractData::Album(_) => None,
                                        })
                                        .collect();
                                    let mut table = txn.open_table(DATA_TABLE).unwrap();

                                    hash_list.into_iter().for_each(|hash| {
                                        let mut database =
                                            table.get(&*hash).unwrap().unwrap().value();
                                        database.album.remove(&*album_id);
                                        table.insert(&*hash, database).unwrap();
                                    });
                                    info!("remove album from all data complete");
                                    None
                                }
                            };
                            if let Some(album) = album_opt {
                                album_table.insert(&*album_id, album).unwrap();
                            };
                        });
                    }
                    txn.commit().unwrap();
                    SHOULD_RESET.store(true, Ordering::SeqCst);
                    info!("Album self-updated")
                }
            })
            .await
            .unwrap();
        }
    });
}
