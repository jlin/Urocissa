//! File-system watcher that can be started once and then runs
//! in the background for the lifetime of the program.

use log::{error, info};
use notify::{Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use walkdir::WalkDir;

use std::{
    collections::HashSet,
    path::{Path, PathBuf},
    sync::{
        LazyLock, Mutex,
        atomic::{AtomicBool, Ordering},
    },
};

use crate::tasks::COORDINATOR;
use crate::public::config::PRIVATE_CONFIG;
use crate::{
    public::constant::{VALID_IMAGE_EXTENSIONS, VALID_VIDEO_EXTENSIONS},
    tasks::deduplicate::DeduplicateTask,
};

/// `true` once the watcher has been successfully initialised.
static IS_WATCHING: AtomicBool = AtomicBool::new(false);

/// Holds the watcher so it is never dropped (dropping stops event delivery).
static WATCHER_HANDLE: LazyLock<Mutex<Option<RecommendedWatcher>>> =
    LazyLock::new(|| Mutex::new(None));

/// Initialise the global filesystem watcher (idempotent).
///
/// Subsequent calls return immediately.
///
/// # Errors
/// Propagates any error from [`notify::Watcher::watch`].
pub fn start_watcher_task() -> anyhow::Result<()> {
    // Fast-path: already running.
    if IS_WATCHING.swap(true, Ordering::SeqCst) {
        return Ok(());
    }

    // Build the watcher.
    let mut watcher = new_watcher()?;
    for path in &PRIVATE_CONFIG.sync_path {
        watcher.watch(path, RecursiveMode::Recursive)?;
        info!("Watching path {:?}", path);
    }

    // Store it globally to keep it alive.
    *WATCHER_HANDLE
        .lock()
        .map_err(|err| anyhow::anyhow!("Failed to lock WATCHER_HANDLE mutex: {}", err))? =
        Some(watcher);

    Ok(())
}

fn is_valid_media_file(path: &Path) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.to_lowercase())
        .map(|ext| {
            VALID_IMAGE_EXTENSIONS.contains(&ext.as_str())
                || VALID_VIDEO_EXTENSIONS.contains(&ext.as_str())
        })
        .unwrap_or(false)
}

fn new_watcher() -> notify::Result<RecommendedWatcher> {
    notify::recommended_watcher(move |res: Result<Event, notify::Error>| match res {
        Ok(evt) => {
            match evt.kind {
                EventKind::Create(_) => {
                    let mut files: HashSet<PathBuf> = HashSet::new();

                    for p in evt.paths {
                        if p.is_file() {
                            files.insert(p);
                        } else if p.is_dir() {
                            WalkDir::new(&p)
                                .into_iter()
                                .filter_map(|e| e.ok())
                                .filter(|e| e.file_type().is_file())
                                .for_each(|e| {
                                    files.insert(e.into_path());
                                });
                        }
                    }

                    for file in files {
                        // Check if the file has a valid extension before submitting
                        if is_valid_media_file(&file) {
                            COORDINATOR.execute_detached(DeduplicateTask::new(file));
                        }
                    }
                }

                EventKind::Modify(_) => {
                    let mut files: HashSet<PathBuf> = HashSet::new();

                    for p in evt.paths {
                        if p.is_file() {
                            files.insert(p);
                        }
                    }

                    for file in files {
                        // Check if the file has a valid extension before submitting
                        if is_valid_media_file(&file) {
                            COORDINATOR.execute_detached(DeduplicateTask::new(file));
                        }
                    }
                }

                _ => { /* ignore other kinds */ }
            }
        }
        Err(err) => error!("Watch error: {:#?}", err),
    })
}
