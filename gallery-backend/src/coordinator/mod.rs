use anyhow::Result;
use std::sync::LazyLock;
use tokio::sync::{mpsc, oneshot};
use tokio::task;

pub mod delete;
pub mod index;
pub mod video;
use crate::coordinator::{delete::DeleteTask, index::IndexTask, video::VideoTask};

#[derive(Debug)]
pub enum Task {
    Delete(DeleteTask),
    Video(VideoTask),
    Index(IndexTask),
}

pub static COORDINATOR: LazyLock<Coordinator> = LazyLock::new(|| {
    info!("Coordinator initialized");
    Coordinator::new()
});

/// (Task, optional reply‐channel) travels through the unbounded queue.
type Envelope = (Task, Option<oneshot::Sender<Result<()>>>);

pub struct Coordinator {
    task_tx: mpsc::UnboundedSender<Envelope>,
}

impl Coordinator {
    pub fn new() -> Self {
        let (task_tx, mut task_rx) = mpsc::unbounded_channel::<Envelope>();

        std::thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().unwrap(); // Tokio runtime
            rt.block_on(async move {
                while let Some((task, reply)) = task_rx.recv().await {
                    match task {
                        Task::Delete(task) => spawn_worker(delete::delete_task, task, reply),
                        Task::Video(task) => spawn_worker(video::video_task, task, reply),
                        Task::Index(task) => spawn_worker(index::index_task, task, reply),
                    }
                }
            });
        });

        Coordinator { task_tx }
    }

    /// Fire-and-forget.
    pub fn submit(&self, task: Task) -> Result<()> {
        self.task_tx.send((task, None))?; // sync send
        Ok(())
    }

    /// Fire and get a `oneshot::Receiver` you may `.await`.
    pub fn submit_with_ack(&self, task: Task) -> Result<oneshot::Receiver<Result<()>>> {
        let (tx, rx) = oneshot::channel(); // single-reply channel
        self.task_tx.send((task, Some(tx)))?;
        Ok(rx)
    }
}

/// Runs blocking / CPU-bound work, then answers through the optional sender.
fn spawn_worker<F, T>(f: F, task: T, reply: Option<oneshot::Sender<Result<()>>>)
where
    F: FnOnce(T) -> Result<()> + Send + 'static,
    T: Send + 'static,
{
    tokio::spawn(async move {
        let res = task::spawn_blocking(move || f(task))
            .await
            .expect("blocking task panicked"); // spawn_blocking docs
        if let Some(tx) = reply {
            let _ = tx.send(res); // ignore if receiver dropped
        }
    });
}

/// Extend later if you need global state.
pub struct StateManager;
