use anyhow::Context;
use std::sync::LazyLock;
use tokio::{
    sync::{mpsc, oneshot},
    task,
};

pub mod album;
pub mod delete;
pub mod index;
pub mod remove;
pub mod video;

use album::AlbumTask;
use delete::DeleteTask;
use index::IndexTask;
use remove::RemoveTask;
use video::VideoTask;

use crate::tui::DASHBOARD;

/// One-shot tasks that travel through the queue.
#[derive(Debug)]
pub enum Task {
    Delete(DeleteTask),
    Video(VideoTask),
    Index(IndexTask),
    Album(AlbumTask),
    Remove(RemoveTask),
}

type Envelope = (Task, Option<oneshot::Sender<anyhow::Result<()>>>);

/// Global singleton you can call from anywhere.
pub static COORDINATOR: LazyLock<Coordinator> = LazyLock::new(Coordinator::new);

pub struct Coordinator {
    tx: mpsc::UnboundedSender<Envelope>,
}

impl Coordinator {
    fn new() -> Self {
        let (tx, mut rx) = mpsc::unbounded_channel::<Envelope>();

        // Dedicated OS thread with its own Tokio runtime
        std::thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().expect("start Tokio runtime");
            rt.block_on(async move {
                while let Some((task, reply)) = rx.recv().await {
                    match task {
                        Task::Delete(t) => spawn_io_worker(delete::delete_task, t, reply),
                        Task::Video(t) => spawn_cpu_worker(video::video_task, t, reply),
                        Task::Index(t) => spawn_cpu_worker(index::index_task, t, reply),
                        Task::Album(t) => spawn_io_worker(album::album_task, t, reply),
                        Task::Remove(t) => spawn_io_worker(remove::remove_task, t, reply),
                    }
                }
            });
        });

        Self { tx }
    }

    /// Fire-and-forget.
    pub fn submit(&self, task: Task) -> anyhow::Result<()> {
        self.tx.send((task, None))?;
        Ok(())
    }

    /// Fire and wait for an ACK.
    pub async fn submit_with_ack(&self, task: Task) -> anyhow::Result<()> {
        let (tx, rx) = oneshot::channel();
        self.tx.send((task, Some(tx)))?;
        rx.await.context("worker crashed before ACK")?
    }
}

/// For I/O-bound *async* work.
fn spawn_io_worker<F, T>(f: F, arg: T, reply: Option<oneshot::Sender<anyhow::Result<()>>>)
where
    F: FnOnce(T) -> anyhow::Result<()> + Send + 'static,
    T: Send + 'static,
{
    tokio::spawn(async move {
        let res = task::spawn_blocking(move || f(arg))
            .await
            .expect("blocking task panicked");
        if let Some(tx) = reply {
            let _ = tx.send(res);
        }
    });
}

/// For CPU-bound *sync* work.
pub fn spawn_cpu_worker<F, T>(f: F, arg: T, reply: Option<oneshot::Sender<anyhow::Result<()>>>)
where
    F: FnOnce(T) -> anyhow::Result<()> + Send + 'static,
    T: Send + 'static,
{
    // Run the closure on Rayon's global pool and await its handle on Tokio.
    tokio::spawn(async move {
        DASHBOARD.write().unwrap().increase_pending();
        let res = tokio_rayon::spawn(move || f(arg)).await;
        if let Some(tx) = reply {
            let _ = tx.send(res);
        }
        DASHBOARD.write().unwrap().decrease_pending();
    });
}
