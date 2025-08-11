use crate::public::constant::runtime::INDEX_RUNTIME;
use crate::public::constant::{VALID_IMAGE_EXTENSIONS, VALID_VIDEO_EXTENSIONS};
use crate::{public::config::PRIVATE_CONFIG, workflow::index_for_watch};
use log::{error, info};
use mini_executor::BatchTask;

use notify_debouncer_full::{
    DebounceEventResult, DebouncedEvent, Debouncer, RecommendedCache, new_debouncer,
    notify::{EventKind, RecommendedWatcher, RecursiveMode},
};

use std::{
    collections::HashSet,
    path::{Path, PathBuf},
    sync::{
        LazyLock, Mutex,
        atomic::{AtomicBool, Ordering},
    },
    time::Duration, // <-- needed
};
use walkdir::WalkDir;

static IS_WATCHING: AtomicBool = AtomicBool::new(false);

// Store the Debouncer to keep the watcher alive
static WATCHER_HANDLE: LazyLock<Mutex<Option<Debouncer<RecommendedWatcher, RecommendedCache>>>> =
    LazyLock::new(|| Mutex::new(None));

pub struct StartWatcherTask;

impl BatchTask for StartWatcherTask {
    fn batch_run(_: Vec<Self>) -> impl std::future::Future<Output = ()> + Send {
        async move {
            start_watcher_task();
        }
    }
}

fn start_watcher_task() {
    // Fast-path: already running.
    if IS_WATCHING.swap(true, Ordering::SeqCst) {
        return;
    }

    let roots: Vec<PathBuf> = PRIVATE_CONFIG.sync_path.iter().cloned().collect();

    match start_debounced_watcher(&roots) {
        Ok(deb) => {
            // Keep the debouncer alive
            *WATCHER_HANDLE.lock().unwrap() = Some(deb);
            for path in &PRIVATE_CONFIG.sync_path {
                info!("Watching path {:?}", path);
            }
        }
        Err(e) => {
            IS_WATCHING.store(false, Ordering::SeqCst);
            error!("Failed to start debounced watcher: {e:?}");
        }
    }
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

// Build and start the debounced watcher; keep its guard alive by storing it.
pub fn start_debounced_watcher(
    roots: &[PathBuf],
) -> notify::Result<Debouncer<RecommendedWatcher, RecommendedCache>> {
    // 1 second inactivity window
    let mut debouncer = new_debouncer(
        Duration::from_secs(1),
        None,
        move |res: DebounceEventResult| {
            match res {
                Ok(events) => {
                    // Collect unique files to index after the quiet period
                    let mut to_index: HashSet<PathBuf> = HashSet::new();

                    for DebouncedEvent { event, .. } in events {
                        for path in event.paths {
                            if path.is_file() {
                                if is_valid_media_file(&path) {
                                    to_index.insert(path);
                                }
                            } else if path.is_dir() {
                                // If a whole directory appeared/renamed, crawl it once
                                if matches!(event.kind, EventKind::Create(_) | EventKind::Modify(_))
                                {
                                    for de in WalkDir::new(&path)
                                        .into_iter()
                                        .filter_map(Result::ok)
                                        .filter(|e| e.file_type().is_file())
                                    {
                                        let p = de.into_path();
                                        if is_valid_media_file(&p) {
                                            to_index.insert(p);
                                        }
                                    }
                                }
                            }
                        }
                    }

                    for path in to_index {
                        INDEX_RUNTIME.spawn(index_for_watch(path, None));
                    }
                }
                Err(errs) => {
                    for e in errs {
                        error!("Watch error: {e:?}");
                    }
                }
            }
        },
    )?;

    for root in roots {
        debouncer.watch(root, RecursiveMode::Recursive)?;
    }

    Ok(debouncer)
}
