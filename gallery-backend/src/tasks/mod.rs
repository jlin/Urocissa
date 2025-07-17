use std::sync::LazyLock;

use mini_actor::Actor;

use crate::public::constant::runtime::TOKIO_RUNTIME;

pub mod actor;
pub mod batcher;

pub static COORDINATOR: LazyLock<Actor> = LazyLock::new(|| Actor::new(&TOKIO_RUNTIME));
