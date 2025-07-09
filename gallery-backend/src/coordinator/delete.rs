use anyhow::{Context, Result};
use std::{
    fs,
    path::{Path, PathBuf},
    sync::LazyLock,
    thread,
    time::Duration,
};

use crate::synchronizer::delete::MAX_DELETE_ATTEMPTS;

#[derive(Debug)]
pub struct DeleteTask {
    pub path: PathBuf,
}

static UPLOAD_PATH: LazyLock<PathBuf> =
    LazyLock::new(|| fs::canonicalize("./upload").expect("`./upload` directory must exist"));

pub fn delete_task(task: DeleteTask) -> Result<()> {
    let path = task.path;
    let mut attempts = 0;
    loop {
        attempts += 1;
        match fs::remove_file(&path) {
            Ok(_) => {
                log::info!("Deleted file: {:?}", path);
                return Ok(());
            }
            Err(err) if attempts < MAX_DELETE_ATTEMPTS => {
                log::warn!(
                    "Failed deleting {:?} (attempt {}), retrying in {}ms: {}",
                    path,
                    attempts,
                    100 * attempts,
                    err
                );
                thread::sleep(Duration::from_millis(100 * attempts));
            }
            Err(err) => {
                return Err(err).with_context(|| {
                    format!("Failed deleting {:?} after {} attempts", path, attempts)
                });
            }
        }
    }
}

fn path_starts_with_upload(path: &Path) -> bool {
    match fs::canonicalize(path) {
        Ok(abs_path) => abs_path.starts_with(&*UPLOAD_PATH),
        Err(_) => false,
    }
}
