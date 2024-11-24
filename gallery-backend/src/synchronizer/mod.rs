use album::start_album_channel;
use event::start_event_channel;
use video::start_video_channel;

pub mod album;
pub mod event;
pub mod video;

pub async fn start_sync() -> anyhow::Result<()> {
    start_event_channel();
    start_video_channel();
    start_album_channel();
    Ok(())
}
