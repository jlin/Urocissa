//! looper.rs  – long-lived background workers with ACK support

pub mod flush_query;
pub mod flush_snapshot;
pub mod update_tree;
use anyhow::Result;
use std::{
    collections::HashMap,
    future::pending,
    sync::{Arc, LazyLock},
};
use tokio::{
    sync::{Notify, mpsc, oneshot},
    task,
};

/// Signals you can poke.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Signal {
    UpdateTree,
    FlushTreeSnapshot,
    FlushQuerySnapshot,
}

impl std::fmt::Display for Signal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Signal::UpdateTree => write!(f, "UpdateTree"),
            Signal::FlushTreeSnapshot => write!(f, "FlushTreeSnapshot"),
            Signal::FlushQuerySnapshot => write!(f, "FlushQuerySnapshot"),
        }
    }
}

/// Internal bundle for each Signal.
#[derive(Debug)]
struct Entry {
    notifier: Arc<Notify>,
    ack_tx: mpsc::UnboundedSender<oneshot::Sender<Result<()>>>,
}

/// Global singleton.
pub static LOOPER: LazyLock<Looper> = LazyLock::new(Looper::new);

#[derive(Debug)]
pub struct Looper {
    entries: HashMap<Signal, Entry>,
}

impl Looper {
    fn new() -> Self {
        let (update_tree_ack_sender, update_tree_ack_receiver) = mpsc::unbounded_channel();
        let update_tree_notifier = Arc::new(Notify::new());

        let (flush_tree_ack_sender, flush_tree_ack_receiver) = mpsc::unbounded_channel();
        let flush_tree_notifier = Arc::new(Notify::new());

        let (flush_query_ack_sender, flush_query_ack_receiver) = mpsc::unbounded_channel();
        let flush_query_notifier = Arc::new(Notify::new());

        let mut entries = HashMap::new();
        entries.insert(
            Signal::UpdateTree,
            Entry {
                notifier: update_tree_notifier.clone(),
                ack_tx: update_tree_ack_sender,
            },
        );
        entries.insert(
            Signal::FlushTreeSnapshot,
            Entry {
                notifier: flush_tree_notifier.clone(),
                ack_tx: flush_tree_ack_sender,
            },
        );
        entries.insert(
            Signal::FlushQuerySnapshot,
            Entry {
                notifier: flush_query_notifier.clone(),
                ack_tx: flush_query_ack_sender,
            },
        );
        // ----- Dedicated OS thread + runtime that NEVER exits -------------------------
        std::thread::spawn(move || {
            let rt = tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .expect("tart Tokio runtime failed");

            rt.block_on(async move {
                // start the worker
                register_worker(
                    Signal::UpdateTree,
                    update_tree_notifier,
                    update_tree_ack_receiver,
                    || update_tree::update_task(),
                );

                register_worker(
                    Signal::FlushTreeSnapshot,
                    flush_tree_notifier,
                    flush_tree_ack_receiver,
                    || flush_snapshot::flush_snapshot_task(),
                );

                register_worker(
                    Signal::FlushQuerySnapshot,
                    flush_query_notifier,
                    flush_query_ack_receiver,
                    || flush_query::flush_query_task(),
                );
                // keep runtime alive forever
                pending::<()>().await;
            });
        });

        Self { entries }
    }

    // -------------------------------------------------------------------------------
    // Public API
    // -------------------------------------------------------------------------------
    /// Fire-and-forget poke.
    pub fn notify(&self, sig: Signal) {
        if let Some(e) = self.entries.get(&sig) {
            e.notifier.notify_one();
        }
    }

    /// Async poke **with** acknowledgment.
    pub async fn notify_with_ack(&self, sig: Signal) -> Result<()> {
        let (tx, rx) = oneshot::channel();
        let entry = self
            .entries
            .get(&sig)
            .ok_or_else(|| anyhow::anyhow!("unknown signal"))?;

        entry
            .ack_tx
            .send(tx)
            .map_err(|_| anyhow::anyhow!("worker shut down"))?;

        entry.notifier.notify_one();
        rx.await?
    }
}

/// Spawn a never-ending worker, ACKing everyone who asked.
fn register_worker(
    kind: Signal,
    notifier: Arc<Notify>,
    mut ack_rx: mpsc::UnboundedReceiver<oneshot::Sender<Result<()>>>,
    job: fn() -> Result<()>,
) {
    let name = kind.to_string();
    tokio::spawn(async move {
        loop {
            notifier.notified().await;

            // collect ACK channels first
            let mut acks = Vec::new();
            while let Ok(s) = ack_rx.try_recv() {
                acks.push(s);
            }

            let res = task::spawn_blocking(job)
                .await
                .expect(&format!("{name} panicked"));

            // clone-friendly result
            let err_msg = res.as_ref().err().map(|e| e.to_string());
            for tx in acks {
                let _ = tx.send(match &err_msg {
                    None => Ok(()),
                    Some(msg) => Err(anyhow::anyhow!(msg.clone())),
                });
            }

            if let Some(msg) = err_msg {
                error!("{name} worker failed: {msg}");
            }
        }
    });
}
