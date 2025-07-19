use crate::{
    public::{
        db::tree::TREE, error_data::handle_error,
        structure::database_struct::database::definition::Database,
    },
    tasks::{COORDINATOR, batcher::flush_tree::FlushTreeTask},
};
use anyhow::Result;
use arrayvec::ArrayString;
use mini_executor::Task;
use std::{mem, path::PathBuf};
use tokio::task::spawn_blocking;

pub struct DeduplicateTask {
    pub path: PathBuf,
    pub hash: ArrayString<64>,
}

impl DeduplicateTask {
    pub fn new(path: PathBuf, hash: ArrayString<64>) -> Self {
        Self { path, hash }
    }
}

impl Task for DeduplicateTask {
    type Output = Result<Option<Database>>;

    fn run(self) -> impl std::future::Future<Output = Self::Output> + Send {
        async move {
            spawn_blocking(move || deduplicate_task(self))
                .await
                .expect("blocking task panicked")
                // convert Err into your crate‑error via `handle_error`
                .map_err(|err| handle_error(err.context("Failed to run deduplicate task")))
        }
    }
}

pub fn deduplicate_task(task: DeduplicateTask) -> Result<Option<Database>> {
    let mut database = Database::new(&task.path, task.hash)?;

    let read_table = TREE.api_read_tree();
    // File already in persistent database

    if let Some(guard) = read_table.get(&*database.hash).unwrap() {
        let mut database_exist = guard.value();
        let file_modify = mem::take(&mut database.alias[0]);
        database_exist.alias.push(file_modify);
        COORDINATOR.execute_batch_detached(FlushTreeTask::insert(vec![database_exist]));
        warn!("File already exists in the database:\n{:#?}", database);

        Ok(None)
    } else {
        Ok(Some(database))
    }
}
