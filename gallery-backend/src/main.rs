#[macro_use]
extern crate rocket;
use anyhow::Result;
// --- Make sure all your modules are declared ---

mod operations;
mod process;
mod public;
mod router;
mod tasks;
mod workflow;

use crate::process::initialization::initialize;
use crate::public::constant::runtime::{ROCKET_RUNTIME, WORKER_RUNTIME};
use crate::public::tui::{DASHBOARD, tui_task};
use crate::tasks::looper::start_expire_check_loop;
use crate::tasks::{BATCH_COORDINATOR, COORDINATOR};
use crate::tasks::batcher::start_watcher::StartWatcherTask;
use crate::tasks::batcher::update_tree::UpdateTreeTask;

use public::constant::redb::{ALBUM_TABLE, DATA_TABLE};
use public::db::tree::TREE;
use redb::ReadableTableMetadata;
use rocket::fs::FileServer;
use router::fairing::cache_control_fairing::cache_control_fairing;
use router::fairing::generate_fairing_routes;
use router::{
    delete::generate_delete_routes, get::generate_get_routes, post::generate_post_routes,
    put::generate_put_routes,
};
use std::thread;
use std::time::Instant;

async fn build_rocket() -> rocket::Rocket<rocket::Build> {
    rocket::build()
        .attach(cache_control_fairing())
        .mount(
            "/assets",
            FileServer::from("../gallery-frontend/dist/assets"),
        )
        .mount("/", generate_get_routes())
        .mount("/", generate_post_routes())
        .mount("/", generate_put_routes())
        .mount("/", generate_delete_routes())
        .mount("/", generate_fairing_routes())
}

fn main() -> Result<()> {
    // 建立並啟動 Worker 執行緒
    let worker_handle = thread::spawn(|| {
        info!("Worker thread starting.");
        // 在此執行緒中使用 WORKER_RUNTIME
        WORKER_RUNTIME.block_on(async {
            // 所有背景任務都在這裡執行
            let rx = initialize();
            let start_time = Instant::now();
            let txn = TREE.in_disk.begin_write().unwrap();
            
            // ❗ 保持您原有的日誌語法
            {
                let table = txn.open_table(DATA_TABLE).unwrap();
                info!(duration = &*format!("{:?}", start_time.elapsed()); "Read {} photos/videos from database.", table.len().unwrap());
                let album_table = txn.open_table(ALBUM_TABLE).unwrap();
                info!(duration = &*format!("{:?}", start_time.elapsed()); "Read {} albums from database.", album_table.len().unwrap());
            }

            txn.commit().unwrap();
            BATCH_COORDINATOR.execute_batch_detached(StartWatcherTask);
            BATCH_COORDINATOR.execute_batch_detached(UpdateTreeTask);
            start_expire_check_loop();

            if let Some(sc) = superconsole::SuperConsole::new() {
                WORKER_RUNTIME.spawn(async move {
                    if let Err(e) = tui_task(sc, DASHBOARD.clone(), rx).await {
                        error!("TUI error: {e}");
                    }
                });
            } else {
                error!("Superconsole disabled (no TTY)");
            }

            info!("Worker thread initialized and running. Waiting for tasks or shutdown signal.");
            // 讓 worker 執行緒等待關閉信號，以保持運行
            if let Err(e) = tokio::signal::ctrl_c().await {
                 error!("Failed to listen for ctrl-c in worker: {}", e);
            }
            info!("Worker thread shutting down.");
        });
    });

    // 建立並啟動 Rocket 執行緒
    let rocket_handle = thread::spawn(|| {
        info!("Rocket thread starting.");
        // 在此執行緒中使用 ROCKET_RUNTIME
        if let Err(e) = ROCKET_RUNTIME.block_on(async {
            let rocket_instance = build_rocket().await.ignite().await?;
            let shutdown_handle = rocket_instance.shutdown();

            ROCKET_RUNTIME.spawn(async move {
                tokio::signal::ctrl_c()
                    .await
                    .expect("Failed to install Ctrl-C handler for Rocket");
                info!("Ctrl-C received, shutting down Rocket server gracefully.");
                shutdown_handle.notify();
            });

            if let Err(e) = rocket_instance.launch().await {
                error!("Rocket server failed: {}", e);
                return Err(anyhow::Error::from(e));
            }
            Ok(())
        }) {
            error!("Rocket thread exited with an error: {}", e);
        }
    });

    // 等待兩個執行緒結束
    info!("Main thread waiting for worker and rocket threads to complete.");
    worker_handle.join().expect("Worker thread panicked");
    rocket_handle.join().expect("Rocket thread panicked");

    Ok(())
}