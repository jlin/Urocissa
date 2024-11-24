use super::Tree;
use crate::public::abstract_data::AbstractData;
use crate::public::database_struct::database_timestamp::DataBaseTimestamp;
use crate::public::redb::{ALBUM_TABLE, DATA_TABLE};
use crate::synchronizer::album::ALBUM_QUEUE_SENDER;

use arrayvec::ArrayString;
use log::info;
use rayon::prelude::ParallelSliceMut;
use redb::ReadableTable;
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex, OnceLock};
use std::thread::sleep;
use std::{sync::atomic::Ordering, time::Duration};
use tokio::sync::mpsc::{unbounded_channel, UnboundedSender};

pub static SHOULD_RESET: AtomicBool = AtomicBool::new(false);
pub static ALBUM_WAITING_FOR_MEMORY_UPDATE_SENDER: OnceLock<UnboundedSender<Vec<ArrayString<64>>>> =
    OnceLock::new();

impl Tree {
    pub fn start_loop(&self) {
        let (album_waiting_for_update_sender, mut album_waiting_for_update_receiver) =
            unbounded_channel::<Vec<ArrayString<64>>>();
        ALBUM_WAITING_FOR_MEMORY_UPDATE_SENDER
            .set(album_waiting_for_update_sender)
            .unwrap();

        tokio::task::spawn_blocking(|| {
            let album_waiting_for_update_repository: Arc<Mutex<Vec<ArrayString<64>>>> =
                Arc::new(Mutex::new(Vec::new())); // Vector to store video hash
            let album_waiting_for_update_repository_repository_clone: Arc<
                Mutex<Vec<ArrayString<64>>>,
            > = Arc::clone(&album_waiting_for_update_repository);

            tokio::task::spawn(async move {
                info!("Album thread spawned");
                while let Some(album_id) = album_waiting_for_update_receiver.recv().await {
                    album_waiting_for_update_repository_repository_clone
                        .lock()
                        .expect("events_repository_arc_clone lock error")
                        .extend(album_id);
                }
            });

            loop {
                if SHOULD_RESET.swap(false, Ordering::SeqCst) {
                    let list_of_waiting_for_update_album_id = {
                        let mut album_waiting_for_update_repository_repository_lock =
                            album_waiting_for_update_repository
                                .lock()
                                .expect("events_repository lock error");
                        std::mem::take(&mut *album_waiting_for_update_repository_repository_lock)
                    };

                    if !list_of_waiting_for_update_album_id.is_empty() {
                        info!("obtain list_of_waiting_for_update_album_id")
                    }

                    let table = self
                        .in_disk
                        .begin_read()
                        .unwrap()
                        .open_table(DATA_TABLE)
                        .unwrap();
                    let priority_list =
                        vec!["DateTimeOriginal", "filename", "modified", "scan_time"];
                    let mut data_vec: Vec<DataBaseTimestamp> = table
                        .iter()
                        .unwrap()
                        .map(|guard| {
                            let (_key, value) = guard.unwrap();
                            let database = value.value();
                            DataBaseTimestamp::new(AbstractData::DataBase(database), &priority_list)
                        })
                        .collect();
                    let album_table = self
                        .in_disk
                        .begin_read()
                        .unwrap()
                        .open_table(ALBUM_TABLE)
                        .unwrap();

                    let album_vec: Vec<DataBaseTimestamp> = album_table
                        .iter()
                        .unwrap()
                        .map(|guard| {
                            let (_key, value) = guard.unwrap();
                            let album = value.value();
                            DataBaseTimestamp::new(AbstractData::Album(album), &priority_list)
                        })
                        .collect();
                    data_vec.extend(album_vec);

                    data_vec.par_sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
                    *self.in_memory.write().unwrap() = data_vec;
                    info!("In-memory cache updated.");

                    if !list_of_waiting_for_update_album_id.is_empty() {
                        ALBUM_QUEUE_SENDER
                            .get()
                            .unwrap()
                            .send(list_of_waiting_for_update_album_id)
                            .unwrap();
                    }
                }
                sleep(Duration::from_millis(500))
            }
        });
    }
}
