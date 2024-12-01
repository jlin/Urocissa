use super::TreeSnapshot;
use crate::public::reduced_data::ReducedData;
use redb::TableDefinition;
use std::{sync::OnceLock, time::Instant};
use tokio::sync::{
    mpsc::{unbounded_channel, UnboundedSender},
    Notify,
};

pub static SHOULD_FLUSH_TREE_SNAPSHOT: Notify = Notify::const_new();

pub static TREE_SNAPSHOT_DELETE_QUEUE_SENDER: OnceLock<UnboundedSender<Vec<u128>>> =
    OnceLock::new();

impl TreeSnapshot {
    pub fn start_loop(&'static self) -> tokio::task::JoinHandle<()> {
        tokio::task::spawn(async move {
            let (tree_snapshot_delete_queue_sender, mut tree_snapshot_delete_queue_receiver) =
                unbounded_channel::<Vec<u128>>();
            TREE_SNAPSHOT_DELETE_QUEUE_SENDER
                .set(tree_snapshot_delete_queue_sender)
                .unwrap();
            while let Some(tree_snapshot_delete_queue) =
                tree_snapshot_delete_queue_receiver.recv().await
            {
                tokio::task::spawn_blocking(move || {
                    tree_snapshot_delete_queue
                        .iter()
                        .for_each(|timestamp_delete| {
                            let write_txn = self.in_disk.begin_write().unwrap();
                            let binding = timestamp_delete.to_string();
                            let table_definition: TableDefinition<u64, ReducedData> =
                                TableDefinition::new(&binding);
                            match write_txn.delete_table(table_definition) {
                                Ok(true) => {
                                    info!("Delete tree cache table: {:?}", timestamp_delete)
                                }
                                Ok(false) => {
                                    error!(
                                        "Failed to delete tree cache table: {:?}",
                                        timestamp_delete
                                    )
                                }
                                Err(e) => {
                                    error!(
                                        "Failed to delete tree cache table: {:?}, error: {:?}",
                                        timestamp_delete, e
                                    )
                                }
                            }
                        });
                })
                .await
                .unwrap();
            }
        });
        tokio::task::spawn(async {
            loop {
                SHOULD_FLUSH_TREE_SNAPSHOT.notified().await;
                tokio::task::spawn_blocking(|| loop {
                    if self.in_memory.len() > 0 {
                        let mut timestamp_opt = None;
                        let mut data_vec_opt = None;
                        {
                            if let Some(ref_data) = self.in_memory.iter().next() {
                                timestamp_opt = Some(*ref_data.key());
                                data_vec_opt = Some(ref_data.clone());
                            }
                        }

                        if let Some(timestamp) = timestamp_opt {
                            if let Some(data_vec) = data_vec_opt {
                                let timestamp_string = timestamp.to_string();

                                let timer_start = Instant::now();
                                let txn = self.in_disk.begin_write().unwrap();
                                let table_definition: TableDefinition<u64, ReducedData> =
                                    TableDefinition::new(&timestamp_string);
                                {
                                    let mut table = txn.open_table(table_definition).unwrap();
                                    data_vec.iter().enumerate().for_each(|(index, data)| {
                                        table.insert(index as u64, data).unwrap();
                                    })
                                }
                                txn.commit().unwrap();
                                info!(duration = &*format!("{:?}", timer_start.elapsed());
                                    "Write in-memory cache into disk"
                                );

                                {
                                    self.in_memory.remove(&timestamp_opt.unwrap());
                                    info!(
                                        "{} items remaining in in-memory tree cache",
                                        self.in_memory.len()
                                    );
                                }
                            }
                        }
                    } else {
                        break;
                    }
                })
                .await
                .unwrap();
            }
        })
    }
}
