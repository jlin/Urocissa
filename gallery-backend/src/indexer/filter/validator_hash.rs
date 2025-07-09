use crate::coordinator::delete::DeleteTask;
use crate::coordinator::{COORDINATOR, Task};
use crate::looper::tree::TREE;
use crate::structure::database_struct::database::definition::Database;
use anyhow::{Context, Result, bail};
use arrayvec::ArrayString;
use blake3::Hasher;
use std::mem;
use std::path::PathBuf;
use std::{fs::File, io::Read, path::Path};

pub fn validator(database: &mut Database) -> anyhow::Result<()> {
    let hash = blake3_hasher(&database.source_path()).with_context(|| {
        format!(
            "[validator] Failed to compute hash for {}",
            database.source_path().display()
        )
    })?;

    let read_table = TREE.api_read_tree();
    // File already in persistent database
    if let Some(guard) = read_table.get(&*hash).unwrap() {
        let mut database_exist = guard.value();
        let file_modify = mem::take(&mut database.alias[0]);
        let path_to_delete = PathBuf::from(&file_modify.file);
        database_exist.alias.push(file_modify);
        TREE.insert_tree_api(&vec![database_exist]).unwrap();
        TREE.tree_update();
        COORDINATOR.submit(Task::Delete(DeleteTask::new(path_to_delete)))?;
        bail!("File already exists in the database");
    }
    // New file
    else {
        database.hash = hash;
        Ok(())
    }
}

fn blake3_hasher(file_path: &Path) -> Result<ArrayString<64>> {
    let mut file = File::open(file_path).with_context(|| {
        format!(
            "[blake3_hasher] Failed to open file: {}",
            file_path.display()
        )
    })?;

    let mut hasher = Hasher::new();
    let mut buffer = [0; 512 * 1024];

    loop {
        let bytes_read = file.read(&mut buffer).with_context(|| {
            format!(
                "[blake3_hasher] Failed to read file: {}",
                file_path.display()
            )
        })?;

        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }

    let hash = hasher.finalize();
    Ok(hash.to_hex())
}
