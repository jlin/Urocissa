pub mod flush_query;
pub mod flush_snapshot;
pub mod start_watcher;

use std::{
    collections::HashMap,
    future::pending,
    sync::{Arc, LazyLock},
};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use tokio::{
    runtime::Runtime,
    sync::{Notify, mpsc, oneshot},
    task,
};

use crate::public::constant::runtime::TOKIO_RUNTIME;

/// Every background task handled by the looper.
///
/// **To add a new task**  
/// 1. Create a new variant here.  
/// 2. Add its worker function in [`Signal::task_fn`].
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, EnumIter)]
pub enum Signal {
    FlushTreeSnapshot,
    FlushQuerySnapshot,
    StartWatcher,
}

impl std::fmt::Display for Signal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{self:?}")
    }
}

impl Signal {
    /// Blocking function executed when this signal is received.
    pub const fn task_fn(self) -> fn() -> anyhow::Result<()> {
        match self {
            Signal::FlushTreeSnapshot => flush_snapshot::flush_snapshot_task,
            Signal::FlushQuerySnapshot => flush_query::flush_query_task,
            Signal::StartWatcher => start_watcher::start_watcher_task,
        }
    }
}

/// Runtime state kept for each [`Signal`].
#[derive(Debug)]
struct Entry {
    notifier: Arc<Notify>,
    ack_sender: mpsc::UnboundedSender<oneshot::Sender<anyhow::Result<()>>>,
}

/// Global singleton that multiplexes [`Signal`] notifications.
pub static LOOPER: LazyLock<Looper> = LazyLock::new(|| Looper::new(&TOKIO_RUNTIME));

#[derive(Debug)]
pub struct Looper {
    entries: HashMap<Signal, Entry>,
}

impl Looper {
    /// Build the singleton and spawn one worker per [`Signal`].
    fn new(rt: &'static Runtime) -> Self {
        let mut entries = HashMap::new();
        let mut workers = Vec::new();

        for signal in Signal::iter() {
            let (ack_sender, ack_receiver) = mpsc::unbounded_channel();
            let notifier = Arc::new(Notify::new());

            entries.insert(
                signal,
                Entry {
                    notifier: notifier.clone(),
                    ack_sender,
                },
            );

            workers.push((signal, notifier, ack_receiver));
        }

        // Private Tokio runtime held alive in its own OS thread.
        std::thread::spawn(move || {
            rt.spawn(async move {
                for (signal, notifier, ack_receiver) in workers {
                    register_worker(signal, notifier, ack_receiver, signal.task_fn());
                }
                pending::<()>().await; // never exit
            });
        });

        Self { entries }
    }

    // ---------------------------------------------------------------------
    // Public API
    // ---------------------------------------------------------------------

    /// Fire-and-forget.
    pub fn notify(&self, signal: Signal) {
        if let Some(entry) = self.entries.get(&signal) {
            entry.notifier.notify_one();
        }
    }

    /// Notify the worker and await an acknowledgement.
    pub async fn notify_with_ack(&self, signal: Signal) -> anyhow::Result<()> {
        let (response_sender, response_receiver) = oneshot::channel();
        let entry = self
            .entries
            .get(&signal)
            .ok_or_else(|| anyhow::anyhow!("unknown signal"))?;

        entry
            .ack_sender
            .send(response_sender)
            .map_err(|_| anyhow::anyhow!("worker shut down"))?;

        entry.notifier.notify_one();
        response_receiver.await?
    }
}

/// Spawn a perpetual async task that waits for `signal` notifications and
/// executes `task_fn` on Tokio’s blocking thread-pool.
fn register_worker(
    signal: Signal,
    notifier: Arc<Notify>,
    mut ack_receiver: mpsc::UnboundedReceiver<oneshot::Sender<anyhow::Result<()>>>,
    task_fn: fn() -> anyhow::Result<()>,
) {
    let worker_name = signal.to_string();
    tokio::spawn(async move {
        loop {
            notifier.notified().await;

            // Drain pending acknowledgements.
            let mut pending_ack_senders = Vec::new();
            while let Ok(sender) = ack_receiver.try_recv() {
                pending_ack_senders.push(sender);
            }

            let job_result = task::spawn_blocking(task_fn)
                .await
                .expect(&format!("{worker_name} panicked"));

            // Fan-out the result.
            let error_message = job_result.as_ref().err().map(|e| e.to_string());
            for sender in pending_ack_senders {
                let _ = sender.send(match &error_message {
                    None => Ok(()),
                    Some(msg) => Err(anyhow::anyhow!(msg.clone())),
                });
            }

            if let Some(msg) = error_message {
                error!("{worker_name} worker failed: {msg}");
            }
        }
    });
}
