use std::sync::LazyLock;

use rayon::{ThreadPool, ThreadPoolBuilder};
use tokio::runtime::{Builder, Runtime};

// 1. 🚀 Rocket 專用的 Tokio Runtime
//    這個 Runtime 專門處理網路請求，執行緒名稱清楚標示。
pub static ROCKET_RUNTIME: LazyLock<Runtime> = LazyLock::new(|| {
    Builder::new_multi_thread()
        .worker_threads(4) // 可以根據您的伺服器需求調整
        .thread_name("rocket-io-worker")
        .enable_all()
        .build()
        .expect("Failed to build Rocket Tokio runtime")
});

// 2. 🛠️ 背景 Worker 專用的 Tokio Runtime
//    這個 Runtime 處理所有非網路的非同步任務，例如檔案監控、資料庫初始化、TUI 等。
pub static WORKER_RUNTIME: LazyLock<Runtime> = LazyLock::new(|| {
    Builder::new_multi_thread()
        .worker_threads(4) // 可以根據您的背景任務負載調整
        .thread_name("background-task-worker")
        .enable_all()
        .build()
        .expect("Failed to build Worker Tokio runtime")
});

// 3. 🧠 計算密集型任務專用的 Rayon 線程池
//    這個線程池專門給 IndexTask 這類 CPU 密集型任務使用。
//    它不會建立全域 (process-level) 的 Rayon 池，因此不會干擾其他執行緒。
pub static WORKER_RAYON_POOL: LazyLock<ThreadPool> = LazyLock::new(|| {
    ThreadPoolBuilder::new()
        .num_threads(4) // 您可以精確控制用於索引的核心數
        .thread_name(|i| format!("cpu-intensive-worker-{}", i))
        .build()
        .expect("Failed to build Worker Rayon pool")
});
