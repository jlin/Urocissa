use crate::{
    public::{
        db::tree::TREE, error_data::handle_error,
        structure::database_struct::database::definition::Database,
    },
    tasks::{COORDINATOR, batcher::flush_tree::FlushTreeTask},
};
use anyhow::Result;
use mini_coordinator::Task;
use path_clean::PathClean;
use std::{mem, path::PathBuf};
use tokio::task::spawn_blocking;

pub struct DeduplicateTask {
    pub path: PathBuf,
}

impl DeduplicateTask {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }
}

impl Task for DeduplicateTask {
    type Output = Result<Option<Database>>;

    fn run(self) -> impl std::future::Future<Output = Self::Output> + Send {
        async move {
            spawn_blocking(move || deduplicate_task(self.path))
                .await
                .expect("blocking task panicked")
                // convert Err into your crate‑error via `handle_error`
                .map_err(|err| handle_error(err.context("Failed to run deduplicate task")))
        }
    }
}

pub fn deduplicate_task(path: PathBuf) -> Result<Option<Database>> {
    let path = path.clean();
    let mut database = Database::new(&path)?;

    let read_table = TREE.api_read_tree();
    // File already in persistent database

    if let Some(guard) = read_table.get(&*database.hash).unwrap() {
        let mut database_exist = guard.value();
        let file_modify = mem::take(&mut database.alias[0]);
        database_exist.alias.push(file_modify);
        COORDINATOR.execute_batch_detached(FlushTreeTask::new(vec![database_exist]));
        warn!("File already exists in the database:\n{:#?}", database);

        Ok(None)
    } else {
        Ok(Some(database))
    }
}
