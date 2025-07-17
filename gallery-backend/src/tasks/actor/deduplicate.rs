use std::{mem, path::PathBuf};

use anyhow::bail;
use path_clean::PathClean;
use tokio::task::spawn_blocking;

use crate::{
    public::{db::tree::TREE, structure::database_struct::database::definition::Database},
    tasks::{
        COORDINATOR,
        actor::{copy::CopyTask, delete::DeleteTask},
        batcher::update_tree::UPDATE_TREE_QUEUE,
    },
};
use mini_actor::Task;
pub struct DeduplicateTask {
    pub path: PathBuf,
}

impl DeduplicateTask {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }
}

impl Task for DeduplicateTask {
    type Output = anyhow::Result<()>;

    fn run(self) -> impl std::future::Future<Output = Self::Output> + Send {
        async move {
            // Spawn the blocking work onto a dedicated thread pool
            let result = spawn_blocking(move || deduplicate_task(self.path))
                .await
                .expect("blocking task panicked");
            result
        }
    }
}

pub fn deduplicate_task(path: PathBuf) -> anyhow::Result<()> {
    let path = path.clean();
    let mut database = Database::new(&path)?;
    let read_table = TREE.api_read_tree();
    // File already in persistent database

    if let Some(guard) = read_table.get(&*database.hash).unwrap() {
        let mut database_exist = guard.value();
        let file_modify = mem::take(&mut database.alias[0]);
        let path_to_delete = PathBuf::from(&file_modify.file);
        database_exist.alias.push(file_modify);
        TREE.insert_tree_api(&vec![database_exist]).unwrap();
        UPDATE_TREE_QUEUE.update(vec![()]);
        COORDINATOR.execute_detached(DeleteTask::new(path_to_delete));
        bail!(
            "File already exists in the database: {:?}",
            database.source_path()
        );
    } else {
        COORDINATOR.execute_detached(CopyTask::new(database));
    }
    Ok(())
}
