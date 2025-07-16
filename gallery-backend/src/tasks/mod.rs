use mini_actor::Actor;
use std::sync::LazyLock;
use tokio::sync::{mpsc::UnboundedReceiver, oneshot};

pub mod actor;
pub mod batcher;
pub mod looper;

use crate::{
    operations::initialization::{
        ffmpeg::check_ffmpeg_and_ffprobe, folder::initialize_folder, logger::initialize_logger,
        redb::initialize_file,
    },
    public::{constant::runtime::TOKIO_RUNTIME, tui::DASHBOARD},
};

pub static COORDINATOR: LazyLock<Actor> = LazyLock::new(|| Actor::new(&TOKIO_RUNTIME));

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

pub fn initialize() -> UnboundedReceiver<String> {
    let rx = initialize_logger();
    check_ffmpeg_and_ffprobe();
    initialize_folder();
    initialize_file();
    rx
}
