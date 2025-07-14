use std::sync::LazyLock;

use tokio::runtime::{Builder, Runtime};

pub static TOKIO_RUNTIME: LazyLock<Runtime> = LazyLock::new(|| {
    Builder::new_multi_thread()
        .worker_threads(2) // Or more, depending on your needs
        .thread_name("my-global-runtime")
        .enable_all()
        .build()
        .expect("Failed to create Tokio runtime")
});