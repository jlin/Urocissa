use album::start_album_channel;
use event::start_event_channel;
use rocket::Shutdown;
use video::start_video_channel;
use watch::start_watcher;

use crate::public::tree::TREE;

pub mod album;
pub mod event;
pub mod video;
pub mod watch;
pub async fn start_sync(shutdown: Shutdown) {
    // Start all tasks
    let task1 = start_event_channel();
    let task2 = start_video_channel();
    let task3 = start_album_channel();
    let task4 = TREE.start_loop();
    let task5 = start_watcher();

    info!("All channels started.");

    tokio::select! {
        res = task1 => {
            match res {
                Ok(_) => {
                    error!("Event channel closed unexpectedly.");
                    shutdown.notify();
                },
                Err(e) => {
                    error!("Event channel task failed: {:?}", e);
                    shutdown.notify();
                },
            }
        },
        res = task2 => {
            match res {
                Ok(_) => {
                    error!("Video channel closed unexpectedly.");
                    shutdown.notify();
                },
                Err(e) => {
                    error!("Video channel task failed: {:?}", e);
                    shutdown.notify();
                },
            }
        },
        res = task3 => {
            match res {
                Ok(_) => {
                    error!("Album channel closed unexpectedly.");
                    shutdown.notify();
                },
                Err(e) => {
                    error!("Album channel task failed: {:?}", e);
                    shutdown.notify();
                },
            }
        },
        res = task4 => {
            match res {
                Ok(_) => {
                    error!("Tree loop closed unexpectedly.");
                    shutdown.notify();
                },
                Err(e) => {
                    error!("Tree loop task failed: {:?}", e);
                    shutdown.notify();
                },
            }
        },
        res = task5 => {
            match res {
                Ok(_) => {
                    error!("Watcher closed unexpectedly.");
                    shutdown.notify();
                },
                Err(e) => {
                    error!("Watcher task failed: {:?}", e);
                    shutdown.notify();
                },
            }
        }
    }
}
