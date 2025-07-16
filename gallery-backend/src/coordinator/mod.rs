use std::sync::LazyLock;
use tokio::sync::oneshot;

pub mod actor;
pub mod album;
pub mod copy;
pub mod deduplicate;
pub mod delete;
pub mod index;
pub mod remove;
pub mod video;

use crate::{constant::runtime::TOKIO_RUNTIME, coordinator::actor::Actor, tui::DASHBOARD};

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
