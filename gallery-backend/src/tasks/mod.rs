use crate::public::constant::runtime::TOKIO_RUNTIME;
use mini_executor::TaskExecutor;
use std::sync::LazyLock;

pub mod actor;
pub mod batcher;

pub static COORDINATOR: LazyLock<TaskExecutor> =
    LazyLock::new(|| TaskExecutor::new(&TOKIO_RUNTIME));
