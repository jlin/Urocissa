use crate::{
    public::{
        db::tree::TREE, error_data::handle_error,
        structure::database_struct::database::definition::Database,
    },
    tasks::{COORDINATOR, actor::delete::DeleteTask, batcher::flush_tree::FLUSH_TREE_QUEUE},
};
use anyhow::Result;
use anyhow::bail;
use mini_actor::Task;
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
    type Output = Result<Database>;

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

pub fn deduplicate_task(path: PathBuf) -> Result<Database> {
    let path = path.clean();
    let mut database = Database::new(&path)?;
    let read_table = TREE.api_read_tree();
    // File already in persistent database

    if let Some(guard) = read_table.get(&*database.hash).unwrap() {
        let mut database_exist = guard.value();
        let file_modify = mem::take(&mut database.alias[0]);
        let path_to_delete = PathBuf::from(&file_modify.file);
        database_exist.alias.push(file_modify);
        FLUSH_TREE_QUEUE.update(vec![database_exist]);
        COORDINATOR.execute_detached(DeleteTask::new(path_to_delete));
        bail!(
            "File already exists in the database: {:?}",
            database.source_path()
        );
    }
    Ok(database)
}
