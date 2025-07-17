use anyhow::Context;
use anyhow::Result;
use mini_coordinator::Task;
use std::fs;
use std::io;
use std::path::Path;
use std::thread;
use std::time::Duration;
use tokio::task::spawn_blocking;

use crate::public::error_data::handle_error;
use crate::public::structure::database_struct::database::definition::Database;

pub struct CopyTask {
    pub database: Database,
}

impl CopyTask {
    pub fn new(database: Database) -> Self {
        Self { database }
    }
}

impl Task for CopyTask {
    type Output = Result<Database>;

    fn run(self) -> impl std::future::Future<Output = Self::Output> + Send {
        async move {
            spawn_blocking(move || copy_task(self.database))
                .await
                .expect("blocking task panicked")
                .map_err(|err| handle_error(err.context("Failed to run copy task")))
        }
    }
}

pub fn copy_task(database: Database) -> Result<Database> {
    let source_path = database.source_path();
    let dest_path = database.imported_path();

    if let Some(parent) = dest_path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("failed to create directory tree for {:?}", parent))?;
    }

    robust_copy(&source_path, &dest_path).with_context(|| {
        format!(
            "failed to copy file from {:?} to {:?}",
            source_path, dest_path
        )
    })?; // 若三次都失敗就進 Err 流程

    Ok(database)
}

fn robust_copy(src: &Path, dst: &Path) -> io::Result<u64> {
    const MAX_RETRIES: u32 = 3;

    for attempt in 0..=MAX_RETRIES {
        match fs::copy(src, dst) {
            Ok(bytes) => return Ok(bytes), // 成功就提早結束
            Err(_) if attempt < MAX_RETRIES => {
                thread::sleep(Duration::from_secs(1)); // 阻塞 1 s 
                continue; // 進入下一回合
            }
            Err(e) => return Err(e), // 第 4 次仍失敗 → 回傳錯誤
        }
    }
    unreachable!("loop guarantees return")
}
