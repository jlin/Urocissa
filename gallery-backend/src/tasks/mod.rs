use crate::public::constant::runtime::{BATCHER_RUNTIME, ROCKET_RUNTIME, WORKER_RUNTIME};
use mini_executor::TaskExecutor;
use std::sync::LazyLock;

pub mod actor;
pub mod batcher;
pub mod looper;

pub static COORDINATOR: LazyLock<TaskExecutor> =
    LazyLock::new(|| TaskExecutor::new(&WORKER_RUNTIME));

pub static BATCH_COORDINATOR: LazyLock<TaskExecutor> =
    LazyLock::new(|| TaskExecutor::new(&BATCHER_RUNTIME));
