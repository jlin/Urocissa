#[macro_use]
extern crate rocket;

// --- Make sure all your modules are declared ---
mod batcher;
mod constant;
mod coordinator;
mod db;
mod indexer;
mod initialization;
mod looper;
mod public;
mod router;
mod structure;
mod tui;
mod utils;

use crate::coordinator::{COORDINATOR, Coordinator};
use crate::initialization::{
    check_ffmpeg_and_ffprobe, initialize_file, initialize_folder, initialize_logger,
};
use crate::looper::{LOOPER, Looper, Signal};
use crate::tui::{DASHBOARD, tui_task};
use constant::redb::{ALBUM_TABLE, DATA_TABLE};
use db::tree::TREE;
use redb::ReadableTableMetadata;
use rocket::fs::FileServer;
use router::fairing::cache_control_fairing::cache_control_fairing;
use router::fairing::generate_fairing_routes;
use router::{
    delete::generate_delete_routes, get::generate_get_routes, post::generate_post_routes,
    put::generate_put_routes,
};
use std::sync::LazyLock;
use std::time::Instant;
use tokio::runtime::{Builder, Runtime};

// --- 1. Define a SINGLE global Tokio runtime ---
// This runtime will be shared by Rocket and all background tasks.
pub static APP_RUNTIME: LazyLock<Runtime> = LazyLock::new(|| {
    Builder::new_multi_thread()
        .worker_threads(4) // Adjust based on your total needs
        .thread_name("app-global-runtime")
        .enable_all()
        .build()
        .expect("Failed to create Tokio runtime")
});

// --- 2. Your rocket building logic, without the #[launch] macro ---
// It's now a regular async function that returns a configured Rocket instance.
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

// --- 3. The new main entry point for your application ---
fn main() -> anyhow::Result<()> {
    // block_on is used to start the async world from our synchronous main function.
    APP_RUNTIME.block_on(async {
        // --- All your initialization logic goes here ---
        let rx = initialize_logger();
        check_ffmpeg_and_ffprobe();
        initialize_folder();
        initialize_file();
        let start_time = Instant::now();
        let txn = TREE.in_disk.begin_write().unwrap();
        {
        let table = txn.open_table(DATA_TABLE).unwrap();
        info!(duration = &*format!("{:?}", start_time.elapsed()); "Read {} photos/vidoes from database.", table.len().unwrap());
        let album_table = txn.open_table(ALBUM_TABLE).unwrap();
        info!(duration = &*format!("{:?}", start_time.elapsed()); "Read {} albums from database.", album_table.len().unwrap());
        }
        txn.commit().unwrap();

        // --- Start your background tasks on the shared runtime ---
        // Force-init the lazy statics. The Looper::new() will now use APP_RUNTIME.
        LazyLock::force(&COORDINATOR);
        LazyLock::force(&LOOPER);

        // Notify tasks to start.
        LOOPER.notify(Signal::StartWatcher);
        LOOPER.notify(Signal::UpdateTree);

        // Spawn the TUI task.
        if let Some(sc) = superconsole::SuperConsole::new() {
            APP_RUNTIME.spawn(async move {
                if let Err(e) = tui_task(sc, DASHBOARD.clone(), rx).await {
                    eprintln!("TUI error: {e}");
                }
            });
        } else {
            eprintln!("Superconsole disabled (no TTY)");
        }

        // --- Build and launch Rocket, and set up graceful shutdown ---
        let rocket_instance = build_rocket().await.ignite().await?;
        let shutdown_handle = rocket_instance.shutdown(); // Get the shutdown handle. [3]

        // Spawn a task to listen for Ctrl-C. [4, 7]
        tokio::spawn(async move {
            tokio::signal::ctrl_c()
                .await
                .expect("Failed to install Ctrl-C handler");
            info!("Ctrl-C received, shutting down gracefully.");
            shutdown_handle.notify(); // Notify Rocket to shut down. [1, 9]
        });

        // Launch the Rocket server and wait for it to complete.
        if let Err(e) = rocket_instance.launch().await {
            error!("Rocket server failed: {}", e);
        };

        Ok(())
    })
}
