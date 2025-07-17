use crate::public::constant::MAX_DELETE_ATTEMPTS;
use anyhow::Context;
use anyhow::Result;
use mini_actor::Task;
use std::{
    fs,
    path::{Path, PathBuf},
    sync::LazyLock,
    thread,
    time::Duration,
};
use tokio::task::spawn_blocking;

static UPLOAD_PATH: LazyLock<PathBuf> =
    LazyLock::new(|| fs::canonicalize("./upload").expect("`./upload` directory must exist"));

pub struct DeleteTask {
    pub path: PathBuf,
}

impl DeleteTask {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }
}

impl Task for DeleteTask {
    type Output = Result<()>;

    fn run(self) -> impl std::future::Future<Output = Self::Output> + Send {
        async move {
            let result = spawn_blocking(move || delete_task(self.path))
                .await
                .expect("blocking task panicked");
            result
        }
    }
}

pub fn delete_task(path: PathBuf) -> Result<()> {
    // Skip if path is not under ./upload
    if !path_starts_with_upload(&path) {
        return Ok(());
    }

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
                return Err(err).context(format!(
                    "Failed deleting {:?} after {} attempts",
                    path, attempts
                ));
            }
        }
    }
}

pub fn path_starts_with_upload(path: &Path) -> bool {
    match fs::canonicalize(path) {
        Ok(abs_path) => abs_path.starts_with(&*UPLOAD_PATH),
        Err(_) => false,
    }
}
