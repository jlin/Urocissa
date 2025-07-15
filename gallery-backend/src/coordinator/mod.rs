use anyhow::Context;
use arrayvec::ArrayString;
use std::{path::PathBuf, sync::LazyLock};
use tokio::{
    runtime::Runtime,
    sync::{mpsc, oneshot},
    task,
};

pub mod actor;
pub mod album;
pub mod copy;
pub mod deduplicate;
pub mod delete;
pub mod index;
pub mod remove;
pub mod video;

use crate::{
    constant::runtime::TOKIO_RUNTIME, structure::database_struct::database::definition::Database,
    tui::DASHBOARD,
};

/// One-shot tasks that travel through the queue.
#[derive(Debug)]
pub enum Task {
    Deduplicate(PathBuf),
    Delete(PathBuf),
    Video(Database),
    Index(Database),
    Album(ArrayString<64>),
    Remove(u128),
    Copy(Database),
}

type Envelope = (Task, Option<oneshot::Sender<anyhow::Result<()>>>);

/// Global singleton you can call from anywhere.
pub static COORDINATOR: LazyLock<Coordinator> = LazyLock::new(|| Coordinator::new(&TOKIO_RUNTIME));

pub struct Coordinator {
    tx: mpsc::UnboundedSender<Envelope>,
}

impl Coordinator {
    fn new(rt: &'static Runtime) -> Self {
        let (tx, mut rx) = mpsc::unbounded_channel::<Envelope>();

        // Dedicated OS thread with its own Tokio runtime
        std::thread::spawn(move || {
            rt.spawn(async move {
                while let Some((task, reply)) = rx.recv().await {
                    match task {
                        Task::Deduplicate(t) => {
                            spawn_io_worker(deduplicate::deduplicate_task, t, reply)
                        }
                        Task::Index(t) => spawn_cpu_worker(index::index_task, t, reply),
                        Task::Copy(t) => spawn_io_worker(copy::copy_task, t, reply),
                        Task::Video(t) => spawn_cpu_worker(video::video_task, t, reply),
                        Task::Delete(t) => spawn_io_worker(delete::delete_task, t, reply),
                        Task::Remove(t) => spawn_io_worker(remove::remove_task, t, reply),
                        Task::Album(t) => spawn_io_worker(album::album_task, t, reply),
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
        DASHBOARD.increase_pending();
        let res = tokio_rayon::spawn(move || f(arg)).await;
        if let Some(tx) = reply {
            let _ = tx.send(res);
        }
        DASHBOARD.decrease_pending();
    });
}
