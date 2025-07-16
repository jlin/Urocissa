use tokio::{runtime::Runtime, task::JoinHandle};

pub trait Task: Sized + Send + 'static {
    type Output: Send + 'static;

    /// Starts this task and returns a Future
    fn run(self) -> impl std::future::Future<Output = Self::Output> + Send;
}

pub struct Actor {
    rt: &'static Runtime,
}

impl Actor {
    /// Creates a new Actor bound to the given runtime
    pub fn new(rt: &'static Runtime) -> Self {
        Actor { rt }
    }

    /// Executes the task and waits for it to complete, returning its output
    pub async fn execute_waiting<T: Task>(&self, task: T) -> T::Output {
        let handle = self.rt.spawn(async move { task.run().await });
        handle.await.unwrap()
    }

    /// Executes the task without waiting, returning a JoinHandle for later awaiting
    pub fn execute_detached<T: Task>(&self, task: T) -> JoinHandle<T::Output> {
        self.rt.spawn(async move { task.run().await })
    }
}
