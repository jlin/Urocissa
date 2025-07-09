use log::info;
use notify::{Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};

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
    let watcher: RecommendedWatcher =
        notify::recommended_watcher(move |watcher_result: notify::Result<Event>| {
            match watcher_result {
                Ok(wacher_events) => {
                    match wacher_events.kind {
                        EventKind::Create(_) => {
                            if !wacher_events.paths.is_empty() {
                                for path in wacher_events.paths {
                                    if let Err(err) =
                                        COORDINATOR.submit(Task::Index(IndexTask::new(path)))
                                    {
                                        error!("Failed to submit task:\n{:#}", err);
                                    }
                                }
                            }
                        }
                        EventKind::Modify(_) => {
                            // Avoid modifying files within the folder to prevent a full rescan of the entire folder
                            let filtered_paths: Vec<PathBuf> = wacher_events
                                .paths
                                .into_iter()
                                .filter(|path| path.is_file())
                                .collect();

                            if !filtered_paths.is_empty() {
                                for path in filtered_paths {
                                    if let Err(err) =
                                        COORDINATOR.submit(Task::Index(IndexTask::new(path)))
                                    {
                                        error!("Failed to submit task:\n{:#}", err);
                                    }
                                }
                            }
                        }
                        _ => (),
                    }
                }
                Err(err) => error!("watch error: {:#?}", err),
            }
        })
        .unwrap();
    watcher
}
