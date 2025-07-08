use std::sync::LazyLock;

use tokio::sync::mpsc;

use crate::coordinator::{delete::DeleteTask, video::VideoTask};

pub mod delete;
pub mod video;

#[derive(Debug)]
pub enum Task {
    Delete(DeleteTask),
    Video(VideoTask),
}

pub static COORDINATOR: LazyLock<Coordinator> = LazyLock::new(|| Coordinator::new());

pub struct Coordinator {
    task_sender: mpsc::UnboundedSender<Task>,
}

impl Coordinator {
    pub fn new() -> Self {
        let (task_sender, mut task_receiver) = mpsc::unbounded_channel::<Task>();

        tokio::spawn(async move {
            while let Some(task) = task_receiver.recv().await {
                tokio::task::spawn_blocking(move || match task {
                    Task::Delete(task) => delete::delete_task(task),
                    Task::Video(task) => video::video_task(task),
                });
            }
        });

        Self { task_sender }
    }

    pub fn submit(&self, task: Task) {
        let _ = self.task_sender.send(task);
    }
}

pub struct StateManager {
    // This struct can be used to manage the state of the coordinator if needed
}
