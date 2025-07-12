use std::{mem, path::PathBuf};

use anyhow::bail;
use path_clean::PathClean;

use crate::{
    coordinator::{delete::DeleteTask, index::IndexTask, Task, COORDINATOR},
    db::tree::TREE,
    looper::{Signal, LOOPER},
    structure::database_struct::database::definition::Database,
};

#[derive(Debug)]
pub struct DeduplicateTask {
    pub path: PathBuf,
}

impl DeduplicateTask {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }
}

pub fn deduplicate_task(task: DeduplicateTask) -> anyhow::Result<()> {
    let path = task.path.clean();
    let mut database = Database::new(&path)?;
    let read_table = TREE.api_read_tree();
    // File already in persistent database

    if let Some(guard) = read_table.get(&*database.hash).unwrap() {
        let mut database_exist = guard.value();
        let file_modify = mem::take(&mut database.alias[0]);
        let path_to_delete = PathBuf::from(&file_modify.file);
        database_exist.alias.push(file_modify);
        TREE.insert_tree_api(&vec![database_exist]).unwrap();
        LOOPER.notify(Signal::UpdateTree);
        COORDINATOR.submit(Task::Delete(DeleteTask::new(path_to_delete)))?;
        bail!(
            "File already exists in the database: {:?}",
            database.source_path()
        );
    } else {
        COORDINATOR.submit(Task::Index(IndexTask::new(database)))?;
    }
    Ok(())
}
