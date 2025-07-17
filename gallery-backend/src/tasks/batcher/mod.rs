pub mod expire_check;
pub mod flush_tree;
pub mod flush_tree_snapshot;
pub mod update_tree;

use std::sync::{Arc, OnceLock};
use tokio::sync::{
    Notify,
    mpsc::{UnboundedSender, unbounded_channel},
};

use crate::public::constant::runtime::TOKIO_RUNTIME;

pub struct Queue<T> {
    pub list: Vec<T>,
    pub notify: Option<Arc<Notify>>,
}

pub struct QueueApi<T: Send + 'static> {
    tx_lock: OnceLock<UnboundedSender<Queue<T>>>,
    process: fn(Vec<T>),
}

impl<T: Send + 'static> QueueApi<T> {
    pub const fn new(process: fn(Vec<T>)) -> Self {
        Self {
            tx_lock: OnceLock::new(),
            process,
        }
    }

    #[inline]
    pub fn tx(&self) -> &UnboundedSender<Queue<T>> {
        self.ensure_started();
        self.tx_lock.get().unwrap()
    }

    pub fn update(&self, list: Vec<T>) {
        self.ensure_started();
        let msg = Queue { list, notify: None };
        self.tx().send(msg).unwrap();
    }

    pub async fn update_async(&self, list: Vec<T>) {
        self.ensure_started();
        let notify = Arc::new(Notify::new());
        let msg = Queue {
            list,
            notify: Some(notify.clone()),
        };
        self.tx().send(msg).unwrap();
        notify.notified().await;
    }

    fn ensure_started(&self) {
        use tokio::sync::mpsc::UnboundedReceiver;

        self.tx_lock.get_or_init(|| {
            let (tx, mut rx): (UnboundedSender<_>, UnboundedReceiver<_>) = unbounded_channel();
            let process_fn = self.process;

            TOKIO_RUNTIME.spawn(async move {
                loop {
                    let mut buf: Vec<Queue<T>> = Vec::new();
                    rx.recv_many(&mut buf, usize::MAX).await;
                    tokio::task::spawn_blocking(move || {
                        let mut batch = Vec::new();
                        let mut waiters = Vec::new();

                        for q in buf {
                            batch.extend(q.list);
                            if let Some(n) = q.notify {
                                waiters.push(n);
                            }
                        }

                        (process_fn)(batch);

                        for n in waiters {
                            n.notify_one();
                        }
                    })
                    .await
                    .expect("blocking task panicked");
                }
            });

            tx
        });
    }
}
