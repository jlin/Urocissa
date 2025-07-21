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
use crate::public::constant::runtime::TOKIO_RUNTIME;
use crate::public::tui::{DASHBOARD, tui_task};
use crate::tasks::COORDINATOR;
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
    TOKIO_RUNTIME.block_on(async {
        // 初始化
        let rx = initialize();
        let start_time = Instant::now();
        let txn = TREE.in_disk.begin_write().unwrap();
        {
            let table = txn.open_table(DATA_TABLE).unwrap();
            info!(duration = &*format!("{:?}", start_time.elapsed()); "Read {} photos/videos from database.", table.len().unwrap());
            let album_table = txn.open_table(ALBUM_TABLE).unwrap();
            info!(duration = &*format!("{:?}", start_time.elapsed()); "Read {} albums from database.", album_table.len().unwrap());
        }
        txn.commit().unwrap();
        COORDINATOR.execute_batch_detached(StartWatcherTask);
        COORDINATOR.execute_batch_detached(UpdateTreeTask);
        if let Some(sc) = superconsole::SuperConsole::new() {
            TOKIO_RUNTIME.spawn(async move {
                if let Err(e) = tui_task(sc, DASHBOARD.clone(), rx).await {
                    error!("TUI error: {e}");
                }
            });
        } else {
            error!("Superconsole disabled (no TTY)");
        }

        let rocket_instance = build_rocket().await.ignite().await?;
        let shutdown_handle = rocket_instance.shutdown();

        TOKIO_RUNTIME.spawn(async move {
            tokio::signal::ctrl_c()
                .await
                .expect("Failed to install Ctrl-C handler");
            info!("Ctrl-C received, shutting down gracefully.");
            shutdown_handle.notify();
        });

        let launch_result = rocket_instance.launch().await;


        if let Err(ref e) = launch_result {
            error!("Rocket server failed: {}", e);
        }

      launch_result.map(|_| ()).map_err(anyhow::Error::from)
      
    })
}
