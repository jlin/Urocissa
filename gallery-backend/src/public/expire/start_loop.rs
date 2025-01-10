use super::Expire;
use crate::public::{
    query_snapshot::{PrefetchReturn, QUERY_SNAPSHOT}, tree::start_loop::VERSION_COUNT_TIMESTAMP, tree_snapshot::TREE_SNAPSHOT, utils::get_current_timestamp_u64
};
use rayon::iter::{ParallelBridge, ParallelIterator};
use redb::{ReadableTable, TableDefinition, TableHandle};
use std::{
    sync::{atomic::{AtomicU64, Ordering}, Arc, OnceLock},
    time::Duration,
};
use tokio::{sync::{mpsc::{unbounded_channel, UnboundedSender}, Notify}, time::sleep};


static SHOULD_CHECK_QUERY_EXPIRE_SENDER: OnceLock<UnboundedSender<Option<Arc<Notify>>>> =
    OnceLock::new();

pub static NEXT_EXPIRE_TIME: AtomicU64 = AtomicU64::new(0);

impl Expire {
    pub fn start_loop(&'static self) -> tokio::task::JoinHandle<()> {
        tokio::task::spawn(async move {
            let (should_check_query_expire_sender, mut should_check_query_expire_receiver) =
            unbounded_channel::<Option<Arc<Notify>>>();

            SHOULD_CHECK_QUERY_EXPIRE_SENDER
            .set(should_check_query_expire_sender)
            .unwrap();
            loop {
                let mut buffer = Vec::new();

                should_check_query_expire_receiver
                    .recv_many(&mut buffer, usize::MAX)
                    .await;

                tokio::task::spawn_blocking(move|| {
   
                    let write_txn = QUERY_SNAPSHOT.in_disk.begin_write().unwrap();
                    // Iter over all tables in QUERY_SNAPSHOT
                    write_txn
                        .list_tables()
                        .unwrap()
                        .par_bridge()
                        .for_each(|table_handle| {
                            if let Ok(timestamp) = table_handle.name().parse::<u64>() {
                                if VERSION_COUNT_TIMESTAMP.load(Ordering::Relaxed) > timestamp
                                    && self.expired_check(timestamp)
                                {
                                    // the table in QUERY_SNAPSHOT expired
                                    // perform purge
                                    let binding = timestamp.to_string();
                                    let table_definition: TableDefinition<u64, PrefetchReturn> =
                                        TableDefinition::new(&binding);

                                    let read_txn = QUERY_SNAPSHOT.in_disk.begin_read().unwrap();
                                    let table = read_txn.open_table(table_definition).unwrap();

                                    match write_txn.delete_table(table_handle) {
                                        Ok(true) => {
                                            info!("Delete query cache table: {:?}", timestamp);
                                            // QUERY_SNAPSHOT purge is complete
                                            // TREE_SNAPSHOT is no longer needed
                                            let tree_snapshot_delete_queue: Vec<_> = table
                                            .iter()
                                            .unwrap()
                                            .par_bridge()
                                            .filter_map(|result| {
                                                let (_, value) = result.unwrap();
                                                value.value().map(|prefetch| prefetch.timestamp)
                                            })
                                            .collect();
                                        
                                            TREE_SNAPSHOT.tree_snapshot_delete(tree_snapshot_delete_queue);
                                        }
                                        Ok(false) => {
                                            error!("Failed to delete query cache table: {:?}", timestamp);
                                        }
                                        Err(e) => {
                                            error!(
                                                "Failed to delete query cache table: {:?}, error: {:?}",
                                                timestamp,
                                                e
                                            );
                                        }
                                    }

                                    info!(
                                        "{} items remaining in disk query cache",
                                        write_txn.list_tables().unwrap().count()
                                    );
                                }
                            }
                        });
                    write_txn.commit().unwrap();
                    buffer.iter().for_each(|notify_opt| {
                        if let Some(notify) = notify_opt {
                            notify.notify_one()
                        }
                    });
                })
                .await
                .unwrap();

                let expire_time = NEXT_EXPIRE_TIME.load(Ordering::Relaxed);
                let current_time = get_current_timestamp_u64();

                if expire_time > current_time {
                    let sleep_duration = expire_time - current_time;
                    let duration = Duration::from_millis(sleep_duration);
                    info!("Expire thread sleep {:?}", duration);
                    sleep(duration).await;
                } else {
                    info!("Expire thread sleep until notified.");
                    self.should_check_query_expire()
                }
            }
        })
    }
    pub fn should_check_query_expire(&self) {
        SHOULD_CHECK_QUERY_EXPIRE_SENDER
            .get()
            .unwrap()
            .send(None)
            .unwrap();
    }
    pub async fn should_check_query_expire_async(&self) {
        let notify = Arc::new(Notify::new());
        SHOULD_CHECK_QUERY_EXPIRE_SENDER
            .get()
            .unwrap()
            .send(Some(notify.clone()))
            .unwrap();
        notify.notified().await
    }
}
