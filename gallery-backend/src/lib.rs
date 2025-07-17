#![allow(dead_code)]

use std::any::{Any, TypeId};
use std::future::Future;

use dashmap::DashMap;
use tokio::{
    runtime::Runtime,
    sync::{
        mpsc::{UnboundedSender, unbounded_channel},
        oneshot,
    },
    task::{JoinError, JoinHandle},
};

/// 代表一個可以獨立異步執行的任務。
pub trait Task: Sized + Send + 'static {
    type Output: Send + 'static;

    fn run(self) -> impl Future<Output = Self::Output> + Send;
}

/// 代表一個可以被批次處理的任務。
/// batch_run 函式處理一批任務，完成後不回傳任何值。
pub trait BatchTask: Sized + Send + 'static {
    fn batch_run(list: Vec<Self>) -> impl Future<Output = ()> + Send;
}

// 用於在 Actor 內部通道中傳遞的項目。
// 包含任務本身以及一個可選的 oneshot sender 用於發送「完成信號」。
type BatchItem<BT> = (BT, Option<oneshot::Sender<()>>);

pub struct Actor {
    rt: &'static Runtime,
    batch_senders: DashMap<TypeId, Box<dyn Any + Send + Sync>>,
}

impl Actor {
    pub fn new(rt: &'static Runtime) -> Self {
        let batch_senders = DashMap::new();
        Actor { rt, batch_senders }
    }

    /// 同步執行一個任務並等待其結果。
    pub async fn execute_waiting<T: Task>(&self, task: T) -> Result<T::Output, JoinError> {
        let handle = self.rt.spawn(task.run());
        handle.await
    }

    /// 非同步地執行一個任務，不等待其結果，直接回傳 JoinHandle。
    pub fn execute_detached<T: Task>(&self, task: T) -> JoinHandle<T::Output> {
        self.rt.spawn(task.run())
    }

    /// 非同步地執行一個批次任務，不等待其完成。
    pub fn execute_batch_detached<BT: BatchTask>(&self, batch_task: BT) {
        let key = TypeId::of::<BT>();
        if let Some(sender_any) = self.batch_senders.get(&key) {
            let sender = sender_any
                .downcast_ref::<UnboundedSender<BatchItem<BT>>>()
                .expect("Type mismatch in batch_senders");
            // 對於 detached 模式，我們傳入 None 作為信號發送者
            sender
                .send((batch_task, None))
                .expect("Failed to send batch task");
        } else {
            // 如果此類型的批次處理器不存在，則創建一個新的
            let (tx, mut rx) = unbounded_channel::<BatchItem<BT>>();

            let _handle: JoinHandle<()> = self.rt.spawn(async move {
                loop {
                    // 等待第一個任務到達
                    if let Some((first_task, first_sender)) = rx.recv().await {
                        let mut tasks = vec![first_task];
                        let mut senders = vec![first_sender];

                        // 嘗試立即從通道中取出盡可能多的任務，以組合成一個批次
                        while let Ok((task, sender)) = rx.try_recv() {
                            tasks.push(task);
                            senders.push(sender);
                        }

                        // 執行批次處理
                        BT::batch_run(tasks).await;

                        // 批次處理完成後，通知所有等待的調用者
                        for sender_opt in senders.into_iter() {
                            if let Some(sender) = sender_opt {
                                // 如果接收端已經被丟棄，這裡會回傳錯誤，但我們忽略它
                                let _ = sender.send(());
                            }
                        }
                    } else {
                        // 如果通道已關閉，則退出循環
                        break;
                    }
                }
            });
            // 將新的發送器存入 DashMap
            self.batch_senders.insert(key, Box::new(tx.clone()));
            // 發送第一個任務
            tx.send((batch_task, None))
                .expect("Failed to send initial batch task");
        }
    }

    /// 執行一個批次任務，並等待其所在的批次執行完成。
    pub async fn execute_batch_waiting<BT: BatchTask>(
        &self,
        batch_task: BT,
    ) -> Result<(), oneshot::error::RecvError> {
        let key = TypeId::of::<BT>();
        // 建立一個 oneshot 通道來接收「完成信號」
        let (tx_oneshot, rx_oneshot) = oneshot::channel::<()>();

        if let Some(sender_any) = self.batch_senders.get(&key) {
            let sender = sender_any
                .downcast_ref::<UnboundedSender<BatchItem<BT>>>()
                .expect("Type mismatch in batch_senders");
            // 發送任務，並附上信號回傳的 sender
            sender
                .send((batch_task, Some(tx_oneshot)))
                .expect("Failed to send batch task");
        } else {
            // 如果此類型的批次處理器不存在，則創建一個新的
            let (tx, mut rx) = unbounded_channel::<BatchItem<BT>>();

            let _handle: JoinHandle<()> = self.rt.spawn(async move {
                loop {
                    if let Some((first_task, first_sender)) = rx.recv().await {
                        let mut tasks = vec![first_task];
                        let mut senders = vec![first_sender];

                        while let Ok((task, sender)) = rx.try_recv() {
                            tasks.push(task);
                            senders.push(sender);
                        }

                        BT::batch_run(tasks).await;

                        for sender_opt in senders.into_iter() {
                            if let Some(sender) = sender_opt {
                                let _ = sender.send(());
                            }
                        }
                    } else {
                        break;
                    }
                }
            });
            // 儲存新的發送器
            self.batch_senders.insert(key, Box::new(tx.clone()));
            // 發送第一個任務
            tx.send((batch_task, Some(tx_oneshot)))
                .expect("Failed to send initial batch task");
        }
        // 等待「完成信號」
        rx_oneshot.await
    }
}
