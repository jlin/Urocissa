//! Coordinator – typed-key version (no string literals).

use anyhow::{Context, Result};
use std::{
    collections::HashMap,
    sync::{Arc, LazyLock},
};
use tokio::{
    sync::{Notify, mpsc, oneshot},
    task,
};

pub mod album;
pub mod delete;
pub mod index;
pub mod update;
pub mod video;

use album::AlbumTask;
use delete::DeleteTask;
use index::IndexTask;
use video::VideoTask;

/// Every long-lived “watch” worker you support.
/// Add new variants as you introduce new workers.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Watch {
    Update,
    // Sync,
    // AnyFutureWorker,
}

/// Convert to a human-readable label for logging.
impl std::fmt::Display for Watch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Watch::Update => write!(f, "Update"),
        }
    }
}

/// Logical jobs passed through the queue.
#[derive(Debug)]
pub enum Task {
    Delete(DeleteTask),
    Video(VideoTask),
    Index(IndexTask),
    Album(AlbumTask),
    Update,
}

/// `(Task, optional reply-channel)` that travels through the MPSC queue.
type Envelope = (Task, Option<oneshot::Sender<Result<()>>>);

/// Call `COORDINATOR.submit(Task::…)` from the rest of your program.
pub static COORDINATOR: LazyLock<Coordinator> = LazyLock::new(|| {
    info!("Coordinator initialised");
    Coordinator::new()
});

pub struct Coordinator {
    task_tx: mpsc::UnboundedSender<Envelope>,
    /// Kept for possible future use (e.g. external metrics, tests, etc.).
    #[allow(dead_code)]
    notifiers: HashMap<Watch, Arc<Notify>>,
}

impl Coordinator {
    pub fn new() -> Self {
        // 1. Ordinary task queue.
        let (task_tx, mut task_rx) = mpsc::unbounded_channel::<Envelope>();

        // 2. Typed notifiers.
        let mut notifiers: HashMap<Watch, Arc<Notify>> = HashMap::new();
        let update_notifier = Arc::new(Notify::new());
        notifiers.insert(Watch::Update, update_notifier.clone());

        // 3. Dedicated OS thread that hosts its own Tokio runtime.
        let update_notifier_cloned = update_notifier.clone();
        std::thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().expect("start Tokio runtime");
            rt.block_on(async move {
                // 3-a. One never-ending watch worker.
                register_watch_worker(Watch::Update, update_notifier_cloned, || {
                    update::update_task()
                });

                // 3-b. Regular task loop.
                while let Some((task, reply)) = task_rx.recv().await {
                    match task {
                        Task::Delete(t) => spawn_worker(delete::delete_task, t, reply),
                        Task::Video(t) => spawn_worker(video::video_task, t, reply),
                        Task::Index(t) => spawn_worker(index::index_task, t, reply),
                        Task::Album(t) => spawn_worker(album::album_task, t, reply),

                        Task::Update => {
                            info!("Received Update task – notifying worker.");
                            update_notifier.notify_one();
                            if let Some(tx) = reply {
                                let _ = tx.send(Ok(()));
                            }
                        }
                    }
                }
            });
        });

        Self { task_tx, notifiers }
    }

    /// Fire-and-forget.
    pub fn submit(&self, task: Task) -> Result<()> {
        self.task_tx.send((task, None))?;
        Ok(())
    }

    /// Fire and wait for an ACK.
    pub async fn submit_with_ack(&self, task: Task) -> Result<()> {
        let (tx, rx) = oneshot::channel();
        self.task_tx
            .send((task, Some(tx)))
            .map_err(|e| anyhow::anyhow!("submit failed: {e}"))?;
        rx.await.context("worker crashed before ACK")?
    }
}

// -----------------------------------------------------------------------------
// Helpers
// -----------------------------------------------------------------------------

/// Runs blocking / CPU-bound work and optionally replies.
fn spawn_worker<F, T>(f: F, task: T, reply: Option<oneshot::Sender<Result<()>>>)
where
    F: FnOnce(T) -> Result<()> + Send + 'static,
    T: Send + 'static,
{
    tokio::spawn(async move {
        let res = task::spawn_blocking(move || f(task))
            .await
            .expect("blocking task panicked");
        if let Some(tx) = reply {
            let _ = tx.send(res); // ignore if receiver dropped
        }
    });
}

/// Spawns a never-ending worker triggered by `notifier`.
fn register_watch_worker(kind: Watch, notifier: Arc<Notify>, job: fn() -> Result<()>) {
    let name = kind.to_string();
    tokio::spawn(async move {
        loop {
            notifier.notified().await;
            info!("{name} worker triggered");
            let res = task::spawn_blocking(job).await.expect("{name} panicked");
            if let Err(e) = res {
                error!("{name} worker failed: {e}");
            } else {
                info!("{name} worker completed");
            }
        }
    });
}
