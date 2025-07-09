use log::info;
use notify::{Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use walkdir::WalkDir;

use std::collections::HashSet;
use std::path::PathBuf;
use std::thread;

use crate::coordinator::index::IndexTask;
use crate::coordinator::{COORDINATOR, Task};
use crate::public::config::PRIVATE_CONFIG;
pub fn start_watcher() -> tokio::task::JoinHandle<()> {
    tokio::task::spawn(async {
        tokio::task::spawn_blocking(|| {
            let sync_path_list: &HashSet<PathBuf> = &PRIVATE_CONFIG.sync_path;
            let mut watcher = get_watcher();

            for path in sync_path_list.iter() {
                watcher.watch(&path, RecursiveMode::Recursive).unwrap();
                info!("Watch path {:?}", path);
            }
            thread::park(); // Because the watcher should keep running
        })
        .await
        .unwrap();
    })
}
fn get_watcher() -> RecommendedWatcher {
    notify::recommended_watcher(move |res: Result<Event, notify::Error>| match res {
        Ok(evt) => {
            match evt.kind {
                // === New or changed paths -> (re)index ===
                EventKind::Create(_) | EventKind::Modify(_) => {
                    // ── collect unique file paths ──────────────────────────────
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

                    // ── submit one task per unique file ────────────────────────
                    for file in files {
                        if let Err(e) = COORDINATOR.submit(Task::Index(IndexTask::new(file))) {
                            error!("submit failed: {:#}", e);
                        }
                    }
                }
                _ => { /* ignore other kinds */ }
            }
        }
        Err(e) => error!("watch error: {:#?}", e),
    })
    .expect("failed to create watcher")
}
