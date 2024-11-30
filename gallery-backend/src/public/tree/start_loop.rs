use super::Tree;
use crate::public::abstract_data::AbstractData;
use crate::public::database_struct::database_timestamp::DataBaseTimestamp;
use crate::public::redb::{ALBUM_TABLE, DATA_TABLE};
use crate::synchronizer::album::ALBUM_QUEUE_SENDER;

use arrayvec::ArrayString;
use log::info;
use rayon::prelude::ParallelSliceMut;
use redb::ReadableTable;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::OnceLock;
use tokio::sync::mpsc::{self, unbounded_channel, UnboundedSender};
use tokio::sync::Notify;

pub static ALBUM_WAITING_FOR_MEMORY_UPDATE_SENDER: OnceLock<UnboundedSender<Vec<ArrayString<64>>>> =
    OnceLock::new();

pub static SHOULD_RESET: Notify = Notify::const_new();

pub static VERSION_COUNT: AtomicUsize = AtomicUsize::new(0);

impl Tree {
    pub fn start_loop(&self) -> tokio::task::JoinHandle<()> {
        tokio::task::spawn(async {
            let (album_waiting_for_update_sender, mut album_waiting_for_update_receiver) =
                unbounded_channel::<Vec<ArrayString<64>>>();
            ALBUM_WAITING_FOR_MEMORY_UPDATE_SENDER
                .set(album_waiting_for_update_sender)
                .unwrap();
            loop {
                SHOULD_RESET.notified().await;
                let mut list_of_waiting_for_update_album_id = Vec::new();
                loop {
                    match album_waiting_for_update_receiver.try_recv() {
                        Ok(mut more_files) => {
                            list_of_waiting_for_update_album_id.append(&mut more_files);
                        }
                        Err(mpsc::error::TryRecvError::Empty) => {
                            // No more items are immediately available
                            break;
                        }
                        Err(mpsc::error::TryRecvError::Disconnected) => {
                            // Sender has been dropped; stop collecting
                            break;
                        }
                    }
                }

                let table = self
                    .in_disk
                    .begin_read()
                    .unwrap()
                    .open_table(DATA_TABLE)
                    .unwrap();
                let priority_list = vec!["DateTimeOriginal", "filename", "modified", "scan_time"];
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

                VERSION_COUNT.fetch_add(1, Ordering::SeqCst);
                info!("In-memory cache updated.");

                if !list_of_waiting_for_update_album_id.is_empty() {
                    ALBUM_QUEUE_SENDER
                        .get()
                        .unwrap()
                        .send(list_of_waiting_for_update_album_id)
                        .unwrap();
                    info!("Send queue albums.");
                }
            }
        })
    }
}
