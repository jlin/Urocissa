use std::sync::LazyLock;
use rayon::ThreadPoolBuilder;
use tokio::{sync::oneshot, task};

use crate::tui::DASHBOARD;

static CPU_POOL: LazyLock<rayon::ThreadPool> =
    LazyLock::new(|| ThreadPoolBuilder::new().num_threads(8).build().unwrap());

pub fn spawn_cpu_worker<F, T>(f: F, arg: T, reply: Option<oneshot::Sender<anyhow::Result<()>>>)
where
    F: FnOnce(T) -> anyhow::Result<()> + Send + 'static,
    T: Send + 'static,
{
    task::spawn(async move {
        DASHBOARD.increase_pending();

        // 建立臨時 oneshot 把 Rayon 執行緒中的結果送回 tokio
        let (tx_inner, rx_inner) = oneshot::channel();

        CPU_POOL.spawn_fifo(move || {
            let res = f(arg);          // 執行真正的 CPU 任務
            let _ = tx_inner.send(res); // 忽略 send 失敗
        });

        // 等待 Rayon 執行完，拿到結果
        let res = rx_inner
            .await
            .unwrap_or_else(|_| Err(anyhow::anyhow!("CPU task panicked")));

        if let Some(tx) = reply {
            let _ = tx.send(res);
        }

        DASHBOARD.decrease_pending();
    });
}
