use std::sync::LazyLock;

use tokio::runtime::{Builder, Runtime};

/// The **only** Tokio runtime used by the application.
///
/// Every component (Rocket, Looper, Batcher, Coordinator, TUI)
/// must obtain its async context from this value.
pub static TOKIO_RUNTIME: LazyLock<Runtime> = LazyLock::new(|| {
    // Use at least 4 threads or the full CPU count, whichever is larger.
    // Requires the `num_cpus` crate (`cargo add num_cpus`).
    let workers = num_cpus::get().max(4);

    Builder::new_multi_thread()
        .worker_threads(workers)
        .thread_name("app-runtime")
        // Split the enables so you can later turn time/IO off individually
        .enable_io()
        .enable_time()
        .build()
        .expect("failed to build global Tokio runtime")
});
