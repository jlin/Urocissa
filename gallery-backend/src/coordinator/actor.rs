use std::{path::PathBuf, sync::LazyLock};

use serde_json::value::Index;
use tokio::{runtime::Runtime, task::JoinHandle};

use std::future::Future;

use crate::constant::runtime::TOKIO_RUNTIME;

pub trait Task: Sized + Send + 'static {
    type O: Send + 'static;

    fn perform_task(self) -> impl std::future::Future<Output = Self::O> + std::marker::Send;
}

struct Actor {
    rt: &'static Runtime,
}

impl Actor {
    fn new(rt: &'static Runtime) -> Self {
        Actor { rt }
    }

    async fn perform<T: Task>(&self, task: T) -> T::O {
        let handle = self.rt.spawn(async move { task.perform_task().await });
        handle.await.unwrap()
    }

    fn perform_one_shot<T: Task>(&self, task: T) -> tokio::task::JoinHandle<T::O> {
        self.rt.spawn(async move { task.perform_task().await })
    }
}

pub struct IndexTask(PathBuf);
impl Task for IndexTask {
    type O = String;

    fn perform_task(self) -> impl std::future::Future<Output = Self::O> + Send {
        async move { format!("{}", self.0.display()) }
    }
}

pub static ACTOR: LazyLock<Actor> = LazyLock::new(|| Actor::new(&TOKIO_RUNTIME));

fn testing(path: PathBuf) -> () {
    let task = IndexTask(path);
    ACTOR.perform_one_shot(task);
}
