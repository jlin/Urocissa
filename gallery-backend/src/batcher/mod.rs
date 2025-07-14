//! queue.rs  —  generic batching queue with *lazy self-starting* worker
//! -------------------------------------------------------------------
//! Declare your queue in any module:
//
//! fn handle_photos(batch: Vec<u64>) {
//!     println!("got {} photo IDs", batch.len());
//! }
//!
//! pub static PHOTO_QUEUE: QueueApi<u64> = QueueApi::new(handle_photos);
//!
//! // Use it anywhere:
//! PHOTO_QUEUE.update(vec![1, 2, 3]);
//! PHOTO_QUEUE.update_async(vec![4]).await;

pub mod flush_tree;

use std::sync::{Arc, OnceLock};
use tokio::sync::{
    Notify,
    mpsc::{UnboundedSender, unbounded_channel},
};

use crate::constant::runtime::TOKIO_RUNTIME;

/* ---------------------------------------------------------------------------
1.  Generic queue element
------------------------------------------------------------------------ */
/// Carries a batch plus an optional `Notify` so senders may wait for an
/// acknowledgement.  Neither field imposes extra trait bounds beyond `Send`.
pub struct Queue<T> {
    pub list: Vec<T>,
    pub notify: Option<Arc<Notify>>,
}

/* ---------------------------------------------------------------------------
2.  Public façade
------------------------------------------------------------------------ */
pub struct QueueApi<T: Send + 'static> {
    /// Lazily-initialised sender; public so you can reach it with
    /// `PHOTO_QUEUE.tx()` if you really need direct access.
    tx_lock: OnceLock<UnboundedSender<Queue<T>>>,
    /// User-supplied batch handler
    process: fn(Vec<T>),
}

impl<T: Send + 'static> QueueApi<T> {
    /// **Const** constructor — can be used in a `static` item because a
    /// function pointer is a `const` value :contentReference[oaicite:1]{index=1}.
    pub const fn new(process: fn(Vec<T>)) -> Self {
        Self {
            tx_lock: OnceLock::new(), // `const fn new()` :contentReference[oaicite:2]{index=2}
            process,
        }
    }

    /* --------------  helper to expose the sender if you want it -------------- */
    #[inline]
    pub fn tx(&self) -> &UnboundedSender<Queue<T>> {
        self.ensure_started();
        // Safe unwrap: ensured by `ensure_started`
        self.tx_lock.get().unwrap()
    }

    /* --------------  public API -------------- */

    /// Fire-and-forget batch.
    pub fn update(&self, list: Vec<T>) {
        self.ensure_started();
        let msg = Queue {
            list: list,
            notify: None,
        };
        self.tx().send(msg).unwrap(); // Never blocks :contentReference[oaicite:3]{index=3}
    }

    /// Batch and wait for acknowledgement.
    pub async fn update_async(&self, list: Vec<T>) {
        self.ensure_started();
        let notify = Arc::new(Notify::new()); // One-shot wake-up :contentReference[oaicite:4]{index=4}
        let msg = Queue {
            list: list,
            notify: Some(notify.clone()),
        };
        self.tx().send(msg).unwrap();
        notify.notified().await; // Wait until worker calls `notify_one`
    }

    /* --------------  internal: lazy worker bootstrap -------------- */
    fn ensure_started(&self) {
        use tokio::sync::mpsc::UnboundedReceiver;

        self.tx_lock.get_or_init(|| {
            let (tx, mut rx): (UnboundedSender<_>, UnboundedReceiver<_>) = unbounded_channel(); // No back-pressure :contentReference[oaicite:5]{index=5}
            let process_fn = self.process;

            // Spawn the worker **once** — `OnceLock` guarantees single init
            TOKIO_RUNTIME.spawn(async move {
                loop {
                    let mut buf: Vec<Queue<T>> = Vec::new();
                    // Batch-receive everything currently buffered; avoids
                    // context-switch storms :contentReference[oaicite:6]{index=6}
                    rx.recv_many(&mut buf, usize::MAX).await;
                    tokio::task::spawn_blocking(move || {
                        // 1️⃣ accumulate everything
                        let mut batch = Vec::new();
                        let mut waiters = Vec::new();

                        for q in buf {
                            batch.extend(q.list); // move items in, no reallocs if capacity fits :contentReference[oaicite:2]{index=2}
                            if let Some(n) = q.notify {
                                waiters.push(n);
                            }
                        }

                        // 2️⃣ single user-defined processing step
                        (process_fn)(batch);

                        // 3️⃣ wake everyone that called `update_async`
                        for n in waiters {
                            n.notify_one(); // each gets exactly one permit :contentReference[oaicite:3]{index=3}
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
