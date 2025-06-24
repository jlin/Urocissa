use crate::constant::redb::DATA_TABLE;
use crate::executor::databaser::generate_compressed_video::generate_compressed_video;
use crate::looper::tree::TREE;
use crate::public::error_data::{ErrorData, handle_error};

use arrayvec::ArrayString;
use std::collections::HashSet;
use std::fs;
use std::sync::OnceLock;
use std::{
    error::Error,
    path::{Path, PathBuf},
    thread,
    time::Duration,
};
use tokio;
use tokio::sync::mpsc::{UnboundedSender, unbounded_channel};

pub static DELETE_QUEUE_SENDER: OnceLock<UnboundedSender<Vec<PathBuf>>> = OnceLock::new();

pub fn start_delete_channel() -> tokio::task::JoinHandle<()> {
    let (delete_queue_sender, mut delete_queue_receiver) = unbounded_channel::<Vec<PathBuf>>();
    DELETE_QUEUE_SENDER.set(delete_queue_sender).unwrap();

    tokio::task::spawn(async move {
        while let Some(list_of_delete_path) = delete_queue_receiver.recv().await {
            tokio::task::spawn_blocking(move || {
                // Deduplicate the paths
                let unique_paths: HashSet<_> = list_of_delete_path.into_iter().collect();
                let path_vec: Vec<_> = unique_paths.into_iter().collect();

                todo!("Implement delete logic with retry here");
            })
            .await
            .unwrap();
        }
    })
}

/// How many times we re-attempt a failing delete.
const MAX_RETRIES: u32 = 3;
/// Back-off between attempts: 200 ms, 400 ms, 600 ms …
const BACKOFF_MS: u64 = 200;

/// Try to delete `path`, retrying on failure with linear back-off.
///
/// * Files ⇒ `fs::remove_file`  
/// * Directories ⇒ `fs::remove_dir_all`
fn delete_path_with_retry(path: &Path) -> Result<(), std::io::Error> {
    for attempt in 0..=MAX_RETRIES {
        let result = if path.is_dir() {
            fs::remove_dir_all(path)
        } else {
            fs::remove_file(path)
        };

        match result {
            Ok(()) => return Ok(()),
            Err(e) if attempt < MAX_RETRIES => {
                // Wait a little and try again.
                thread::sleep(Duration::from_millis(BACKOFF_MS * (attempt + 1) as u64));
            }
            Err(e) => return Err(e),
        }
    }
    unreachable!()
}

/// Delete `path` **only if** it lives inside `./upload`.
///
/// Paths are canonicalised first so that `../` tricks cannot escape the sandbox.
pub fn delete_if_in_upload<P: AsRef<Path>>(path: P) -> Result<(), Box<dyn Error>> {
    let upload_root = Path::new("./upload").canonicalize()?;
    let target = path.as_ref().canonicalize()?;

    if target.starts_with(&upload_root) {
        delete_path_with_retry(&target)?;
    }
    Ok(())
}
