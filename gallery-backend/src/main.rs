#[macro_use]
extern crate rocket;
use crate::coordinator::{COORDINATOR, Coordinator};
use crate::db::tree::TREE;
use crate::initialization::initialize_logger;
use crate::looper::{LOOPER, Looper, Signal};
use crate::tui::{DASHBOARD, tui_task};
use constant::redb::{ALBUM_TABLE, DATA_TABLE};
use initialization::{check_ffmpeg_and_ffprobe, initialize_file, initialize_folder};
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
#[launch]
async fn rocket() -> _ {
    /*  let threads = num_cpus::get() * 2;

    ThreadPoolBuilder::new()
        .num_threads(threads)
        .build_global()
        .unwrap(); // 全域設置 */
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

    // Force-init global Coordinator and Looper on their own threads/runtime.
    let _ = LazyLock::<Coordinator>::force(&COORDINATOR);
    let _ = LazyLock::<Looper>::force(&LOOPER);

    LOOPER.notify(Signal::StartWatcher);
    LOOPER.notify(Signal::UpdateTree);

    if let Some(sc) = superconsole::SuperConsole::new() {
        rocket::tokio::spawn(async move {
            // run the non-interactive TUI
            if let Err(e) = tui_task(sc, DASHBOARD.clone(), rx).await {
                eprintln!("TUI error: {e}");
            }
        });
    } else {
        eprintln!("Superconsole disabled (no TTY)");
    }

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
