use std::{
    sync::OnceLock,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use tokio::sync::mpsc::{UnboundedSender, unbounded_channel};

pub fn get_current_timestamp_u64() -> u64 {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    timestamp as u64
}

/// Spawns an asynchronous “collector + flusher” task.
///
/// * **tick_cfg** – `None` ⇒ no timer;  
///   `Some((period, default_val))` ⇒ every *period* the task is called:  
///   • with the current buffer, **or**  
///   • with `[default_val.clone()]` if the buffer is empty.  
///   After a flush triggered by incoming items, the timer is reset,
///   so the next tick is *period* after that flush.
///
/// * **once_lock_sender** – global hook that receives the channel’s sender exactly once.
///   Useful for pushing items into the loop from other parts of the program.
///
/// * **task** – CPU-bound callback run on a blocking thread; receives a
///   `Vec<T>` containing either the batch of received items or the single
///   `default_val`.
///
/// Returns a `JoinHandle<()>` for graceful shutdown.
pub fn start_loop_util<T, F>(
    tick_cfg: Option<(Duration, T)>,
    once_lock_sender: &'static OnceLock<UnboundedSender<T>>,
    task: F,
) -> tokio::task::JoinHandle<()>
where
    T: Send + Clone + 'static,
    F: Fn(Vec<T>) + Copy + Send + Sync + 'static,
{
    tokio::task::spawn(async move {
        let (sender, mut receiver) = unbounded_channel::<T>();
        once_lock_sender.set(sender).unwrap();
        let mut buffer: Vec<T> = Vec::new();

        if let Some((period, default_val)) = tick_cfg {
            let mut ticker = tokio::time::interval(period);
            loop {
                tokio::select! {
                    _ = receiver.recv_many(&mut buffer, usize::MAX) => {
                        if !buffer.is_empty() {
                            let buf = std::mem::take(&mut buffer);
                            tokio::task::spawn_blocking(move || task(buf)).await.unwrap();
                            ticker.reset();
                        }
                    }
                    _ = ticker.tick() => {
                        if buffer.is_empty() {
                            let v = vec![default_val.clone()];
                            tokio::task::spawn_blocking(move || task(v)).await.unwrap();
                        } else {
                            let buf = std::mem::take(&mut buffer);
                            tokio::task::spawn_blocking(move || task(buf)).await.unwrap();
                        }
                    }
                }
            }
        } else {
            loop {
                receiver.recv_many(&mut buffer, usize::MAX).await;
                if !buffer.is_empty() {
                    let buf = std::mem::take(&mut buffer);
                    tokio::task::spawn_blocking(move || task(buf))
                        .await
                        .unwrap();
                }
            }
        }
    })
}
