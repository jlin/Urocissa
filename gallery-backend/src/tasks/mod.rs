use mini_actor::Actor;
use std::sync::LazyLock;

pub mod actor;
pub mod batcher;
pub mod looper;

use crate::public::constant::runtime::TOKIO_RUNTIME;

pub static COORDINATOR: LazyLock<Actor> = LazyLock::new(|| Actor::new(&TOKIO_RUNTIME));
