use crate::public::constant::runtime::{ROCKET_RUNTIME, WORKER_RUNTIME};
use mini_executor::TaskExecutor;
use std::sync::LazyLock;

pub mod actor;
pub mod batcher;
pub mod looper;

pub static COORDINATOR: LazyLock<TaskExecutor> =
    LazyLock::new(|| TaskExecutor::new(&WORKER_RUNTIME));

pub static ROCKET_COORDINATOR: LazyLock<TaskExecutor> =
    LazyLock::new(|| TaskExecutor::new(&ROCKET_RUNTIME));
