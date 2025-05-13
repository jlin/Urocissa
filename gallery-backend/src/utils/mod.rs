use std::{
    sync::OnceLock,
    time::{SystemTime, UNIX_EPOCH},
};

use tokio::sync::mpsc::{unbounded_channel, UnboundedSender};

pub fn get_current_timestamp_u64() -> u64 {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    timestamp as u64
}

pub fn start_loop_util<T: std::marker::Send + 'static>(
    once_lock_sender: &'static OnceLock<UnboundedSender<T>>,
    task: impl FnOnce(Vec<T>) -> () + Send + std::marker::Copy + 'static,
) -> tokio::task::JoinHandle<()> {
    tokio::task::spawn(async move {
        let (sender, mut receiver) = unbounded_channel::<T>();

        once_lock_sender.set(sender).unwrap();

        loop {
            let mut buffer = Vec::new();

            receiver.recv_many(&mut buffer, usize::MAX).await;
            tokio::task::spawn_blocking(move || task(buffer))
                .await
                .unwrap();
        }
    })
}
