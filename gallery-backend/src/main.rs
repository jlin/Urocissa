#[macro_use]
extern crate rocket;
use crate::coordinator::{COORDINATOR, Coordinator};
use crate::db::tree::TREE;
use crate::looper::{LOOPER, Looper, Signal};
use constant::redb::{ALBUM_TABLE, DATA_TABLE};
use initialization::{
    check_ffmpeg_and_ffprobe, initialize_file, initialize_folder, initialize_logger,
};
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
mod utils;
#[launch]
async fn rocket() -> _ {
    initialize_logger();
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
