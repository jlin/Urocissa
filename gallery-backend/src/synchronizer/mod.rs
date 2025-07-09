use rocket::Shutdown;
use watch::start_watcher;

use crate::coordinator::{COORDINATOR, Coordinator};
use crate::looper::{LOOPER, Looper, Signal};
use crate::looper::{expire::EXPIRE, query_snapshot::QUERY_SNAPSHOT};

use futures::stream::{FuturesUnordered, StreamExt};
use log::{error, info};
use std::future::Future;
use std::sync::LazyLock;
use tokio::task::JoinError;
use tokio::task::JoinHandle;

pub mod watch;

/// Define a type alias for better readability
type TaskResult = Result<(), JoinError>;

/// Helper function to associate a task with its name
fn named_task(
    name: &'static str,
    handle: JoinHandle<()>,
) -> impl Future<Output = (&'static str, TaskResult)> {
    async move { (name, handle.await) }
}

pub async fn start_sync(shutdown: Shutdown) {
    let _ = LazyLock::<Coordinator>::force(&COORDINATOR);
    let _ = LazyLock::<Looper>::force(&LOOPER);
    // Initialize a collection of tasks with their respective names
    let mut tasks = FuturesUnordered::new();

    tasks.push(named_task("Expire loop", EXPIRE.start_loop()));
    tasks.push(named_task(
        "Query snapshot loop",
        QUERY_SNAPSHOT.start_loop(),
    ));
    tasks.push(named_task("Watcher", start_watcher()));
    LOOPER.notify(Signal::Update);
    info!("All channels started.");

    // Await the first task to complete
    if let Some((name, result)) = tasks.next().await {
        match result {
            Ok(_) => {
                error!("{} closed unexpectedly.", name);
            }
            Err(err) => {
                error!("{} task failed: {:#?}", name, err);
            }
        }
        // Notify shutdown after any task completes
        shutdown.notify();
    }
}
