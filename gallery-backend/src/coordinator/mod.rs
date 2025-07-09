use anyhow::Context;
use std::sync::LazyLock;
use tokio::sync::{mpsc, oneshot};
use tokio::task;

pub mod album;
pub mod delete;
pub mod index;
pub mod update;
pub mod video;
use crate::coordinator::album::AlbumTask;
use crate::coordinator::{delete::DeleteTask, index::IndexTask, video::VideoTask};

#[derive(Debug)]
pub enum Task {
    Delete(DeleteTask),
    Video(VideoTask),
    Index(IndexTask),
    Album(AlbumTask),
    Update(),
}

pub static COORDINATOR: LazyLock<Coordinator> = LazyLock::new(|| {
    info!("Coordinator initialized");
    Coordinator::new()
});

/// (Task, optional reply‐channel) travels through the unbounded queue.
type Envelope = (Task, Option<oneshot::Sender<anyhow::Result<()>>>);

pub struct Coordinator {
    task_tx: mpsc::UnboundedSender<Envelope>,
}

impl Coordinator {
    pub fn new() -> Self {
        let (task_tx, mut task_rx) = mpsc::unbounded_channel::<Envelope>();

       // 1. 建立一個 Notifier，用 Arc 包裝以便在異步任務間共享
        let update_notifier = Arc::new(Notify::new());

        std::thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async move {
                // 2. 複製 Notifier 給專門處理 update 的任務
                let update_task_notifier = update_notifier.clone();

                // 3. Spawn 一個專門的任務來處理 `Update`
                // 這個任務會一直存在，等待通知
                tokio::spawn(async move {
                    loop {
                        // 等待 `notify_one()` 被呼叫
                        update_task_notifier.notified().await;
                        info!("Update task triggered by notifier, starting execution.");

                        // 執行耗時的 blocking 任務
                        let res = task::spawn_blocking(update::update_task)
                            .await
                            .expect("blocking update task panicked");

                        if let Err(e) = res {
                            // 實際的任務失敗了，最好記錄下來
                            error!("Update task failed: {}", e);
                        } else {
                            info!("Update task completed successfully.");
                        }
                    }
                });

                // 4. 主任務循環
                while let Some((task, reply)) = task_rx.recv().await {
                    match task {
                        Task::Delete(task) => spawn_worker(delete::delete_task, task, reply),
                        Task::Video(task) => spawn_worker(video::video_task, task, reply),
                        Task::Index(task) => spawn_worker(index::index_task, task, reply),
                        Task::Album(task) => spawn_worker(album::album_task, task, reply),
                        
                        // 5. 對於 Update 任務的處理方式改變了
                        Task::Update() => {
                            info!("Received Update task, notifying the handler.");
                            // 只發送一個通知，而不是產生一個新 worker
                            // 如果在 update 任務執行期間有多個通知，它們會被合併成下一次執行
                            update_notifier.notify_one();

                            // 如果調用者需要 ack，我們立即回覆表示「請求已收到」
                            // 注意：這不代表任務已完成
                            if let Some(tx) = reply {
                                // 忽略發送失敗的錯誤 (如果接收端已 dropped)
                                let _ = tx.send(Ok(()));
                            }
                        }
                    }
                }
            });
        });

        Coordinator { task_tx }
    }
    }

    /// Fire-and-forget.
    pub fn submit(&self, task: Task) -> anyhow::Result<()> {
        self.task_tx.send((task, None))?; // sync send
        Ok(())
    }

    /// Fire and get a `oneshot::Receiver` you may `.await`.
    pub async fn submit_with_ack(&self, task: Task) -> anyhow::Result<()> {
        let (tx, rx) = tokio::sync::oneshot::channel();

        self.task_tx
            .send((task, Some(tx)))
            .map_err(|e| anyhow::anyhow!("Failed to submit task to worker queue: {}", e))?;

        let task_result = rx
            .await
            .context("Failed to receive acknowledgment from worker. It might have crashed.")?;

        task_result
    }
}

/// Runs blocking / CPU-bound work, then answers through the optional sender.
fn spawn_worker<F, T>(f: F, task: T, reply: Option<oneshot::Sender<anyhow::Result<()>>>)
where
    F: FnOnce(T) -> anyhow::Result<()> + Send + 'static,
    T: Send + 'static,
{
    tokio::spawn(async move {
        let res = task::spawn_blocking(move || f(task))
            .await
            .expect("blocking task panicked"); // spawn_blocking docs
        if let Some(tx) = reply {
            let _ = tx.send(res); // ignore if receiver dropped
        }
    });
}

/// Extend later if you need global state.
pub struct StateManager;
