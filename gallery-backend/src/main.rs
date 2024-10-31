static BATCH_SIZE: usize = 100;
#[macro_use]
extern crate rocket;
use crate::public::error_data::{handle_error, ErrorData};
use anstyle::Color;
use arrayvec::ArrayString;
use env_logger::fmt::style::Style;
use env_logger::Builder;
use log::kv::Key;
use log::{warn, LevelFilter};
use notify::{Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use public::config::PRIVATE_CONFIG;
use public::redb::DATA_TABLE;
use public::tree::start_loop::SHOULD_RESET;
use public::tree::TREE;
use redb::ReadableTableMetadata;
use rocket::fs::FileServer;
use router::fairing::{auth_request_fairing, cache_control_fairing};
use router::{
    delete::delete_data::delete_data,
    get::get_data::{get_config, get_data, get_data_length, get_rows, get_scroll_bar, get_tags},
    get::get_img::compressed_file,
    get::get_page::{
        all, all_view, archived, archived_view, catch_view_routes, favicon, favorite,
        favorite_view, login, redirect_to_login, redirect_to_photo, redirect_to_photo_2, setting,
        tags, trashed, trashed_view, unauthorized,
    },
    post::{authenticate::authenticate, post_upload::upload},
    put::{
        edit_tag::edit_tag, random::generate_random_data, regenerate_preview::regenerate_preview,
    },
};
use std::io::Write;
use std::sync::atomic::Ordering;
use std::sync::OnceLock;
use std::time::Instant;
use std::{
    panic::Location,
    path::PathBuf,
    sync::{atomic::AtomicBool, Arc, Mutex},
};
use tokio::{sync::mpsc::UnboundedSender, time::Duration};
mod executor;
mod public;
mod router;
mod synchronizer;

static EVENTS_SENDER: OnceLock<UnboundedSender<Vec<PathBuf>>> = OnceLock::new();
static VIDEO_QUEUE_SENDER: OnceLock<UnboundedSender<Vec<ArrayString<64>>>> = OnceLock::new();

#[launch]
async fn rocket() -> _ {
    Builder::new()
        .format(|buf, record| {
            let custom_value = record
                .key_values()
                .get(Key::from("duration"))
                .map(|v| format!("{}", v))
                .unwrap_or_default(); // 如果沒有找到 "duration" 則返回空字串

            // 設置樣式：粗體藍色的 custom_value
            let custom_value_style = Style::new()
                .bold() // 設置粗體
                .fg_color(Some(Color::Ansi(anstyle::AnsiColor::Blue))); // 設置藍色前景

            // 設置樣式：灰色的 timestamp 和 target
            let grey_style =
                Style::new().fg_color(Some(Color::Ansi(anstyle::AnsiColor::BrightBlack))); // 設置灰色前景

            // 開始寫入 buffer，將所有內容合併成一次 write!
            writeln!(
                buf,
                "{}{}{}{} {}{}{} {}{}{}\n{:>10}{} {}",
                // 使用灰色樣式渲染 timestamp
                grey_style.render(),
                buf.timestamp(),           // 直接使用 timestamp 方法
                grey_style.render_reset(), // 重置灰色樣式
                // 渲染 level 的顏色
                buf.default_level_style(record.level()).render(), // 開始渲染 Level 樣式
                record.level(),                                   // 真正的 Level 內容
                buf.default_level_style(record.level()).render_reset(), // 渲染 Level 樣式結束
                // 使用灰色樣式渲染 target
                grey_style.render(),
                record.target(),           // Target 顯示在 Level 之後
                grey_style.render_reset(), // 重置灰色樣式
                // 使用藍色粗體樣式渲染 custom_value
                custom_value_style.render(),
                custom_value,                      // 寫入 custom_value
                custom_value_style.render_reset(), // 重置 custom_value 樣式
                // 寫入 Log 訊息
                record.args()
            )?;

            Ok(())
        })
        .filter(None, LevelFilter::Info) // 設置所有模組的最低 Level 為 Warn
        .filter(Some("rocket"), LevelFilter::Warn)
        .init();
    std::fs::create_dir_all(PathBuf::from("./db")).unwrap();
    let start_time = Instant::now();
    let txn = TREE.in_disk.begin_write().unwrap();
    {
        let table = txn.open_table(DATA_TABLE).unwrap();
        info!(duration = &*format!("{:?}", start_time.elapsed()); "Read {} data from database.", table.len().unwrap());
    }
    txn.commit().unwrap();
    SHOULD_RESET.store(true, Ordering::SeqCst);
    let turn_sync_on = Arc::new(AtomicBool::new(true));
    let turn_sync_on_clone_for_stop = Arc::clone(&turn_sync_on);
    let _import_path: Arc<Mutex<Option<PathBuf>>> = Arc::new(Mutex::new(None));
    let (events_sender, events_receiver) = tokio::sync::mpsc::unbounded_channel::<Vec<PathBuf>>();
    EVENTS_SENDER.set(events_sender).unwrap();

    let (video_queue_sender, video_queue_receiver) =
        tokio::sync::mpsc::unbounded_channel::<Vec<ArrayString<64>>>();
    VIDEO_QUEUE_SENDER.set(video_queue_sender).unwrap();

    let turn_sync_on_clone = Arc::clone(&turn_sync_on_clone_for_stop);

    std::fs::create_dir_all(PathBuf::from("./object/imported")).unwrap();
    std::fs::create_dir_all(PathBuf::from("./object/compressed")).unwrap();
    std::fs::create_dir_all(PathBuf::from("upload")).unwrap();

    tokio::spawn(async move {
        start_watcher().await;
    });
    tokio::spawn(async move {
        synchronizer::start_sync(
            events_receiver,
            video_queue_receiver,
            turn_sync_on_clone.clone(),
        )
        .await
        .expect("start_sync error");
    });
    rocket::build()
        .attach(cache_control_fairing())
        .attach(auth_request_fairing())
        .manage(turn_sync_on_clone_for_stop)
        .mount("/object/imported", FileServer::from("./object/imported"))
        .mount(
            "/assets",
            FileServer::from("../gallery-frontend/dist/assets"),
        )
        .mount(
            "/",
            routes![
                redirect_to_photo,
                redirect_to_photo_2,
                favicon,
                login,
                compressed_file,
                edit_tag,
                tags,
                favorite,
                favorite_view,
                archived,
                archived_view,
                all,
                all_view,
                setting,
                upload,
                get_data,
                get_config,
                get_tags,
                catch_view_routes,
                unauthorized,
                get_data_length,
                generate_random_data,
                get_rows,
                delete_data,
                trashed,
                trashed_view,
                get_scroll_bar,
                regenerate_preview,
                authenticate,
                redirect_to_login
            ],
        )
}

async fn start_watcher() {
    let sync_path_list: &Vec<PathBuf> = &PRIVATE_CONFIG.sync_path;
    let mut watcher: RecommendedWatcher =
        notify::recommended_watcher(move |watcher_result: notify::Result<Event>| {
            match watcher_result {
                Ok(wacher_events) => {
                    match wacher_events.kind {
                        EventKind::Create(_) => {
                            if !wacher_events.paths.is_empty() {
                                if let Err(send_error) = EVENTS_SENDER
                                    .get()
                                    .unwrap()
                                    .send(wacher_events.paths.clone())
                                {
                                    let error_data = ErrorData::new(
                                        format!("Failed to send paths: {}", send_error),
                                        format!(
                                            "Error occur when sending path {:?}",
                                            wacher_events.paths
                                        ),
                                        None,
                                        None,
                                        Location::caller(),
                                    );
                                    handle_error(error_data);
                                }
                            }
                        }
                        EventKind::Modify(_) => {
                            // Avoid modifying files within the folder to prevent a full rescan of the entire folder
                            let filtered_paths: Vec<PathBuf> = wacher_events
                                .paths
                                .into_iter()
                                .filter(|path| path.is_file())
                                .collect();

                            if !filtered_paths.is_empty() {
                                EVENTS_SENDER
                                    .get()
                                    .unwrap()
                                    .send(filtered_paths)
                                    .expect("events_sender send error");
                            }
                        }
                        _ => (),
                    }
                }
                Err(e) => error!("watch error: {:?}", e),
            }
        })
        .unwrap();
    {
        for path in sync_path_list.iter() {
            watcher.watch(&path, RecursiveMode::Recursive).unwrap();
        }
    }
    loop {
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}
