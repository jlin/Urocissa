use album::start_album_channel;
use event::start_event_channel;
use video::start_video_channel;

pub mod album;
pub mod event;
pub mod video;

pub async fn start_sync() {
    // Start all tasks
    let task1 = start_event_channel();
    let task2 = start_video_channel();
    let task3 = start_album_channel();

    info!("All channels started.");

    tokio::select! {
        res = task1 => {
            match res {
                Ok(_) => panic!("Event channel closed unexpectedly."),
                Err(e) => panic!("Event channel task failed: {:?}", e),
            }
        },
        res = task2 => {
            match res {
                Ok(_) => panic!("Video channel closed unexpectedly."),
                Err(e) => panic!("Video channel task failed: {:?}", e),
            }
        },
        res = task3 => {
            match res {
                Ok(_) => panic!("Album channel closed unexpectedly."),
                Err(e) => panic!("Album channel task failed: {:?}", e),
            }
        },
    }
}
