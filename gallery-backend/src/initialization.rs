use anstyle::Color;
use env_logger::fmt::style::Style;
use env_logger::{Builder, WriteStyle};
use log::kv::Key;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use superconsole::style::Stylize;

use std::io::Write;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender, unbounded_channel};

use crate::tui::LOGGER_TX;

pub struct TokioPipe(pub UnboundedSender<String>);
impl std::io::Write for TokioPipe {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let s = String::from_utf8_lossy(buf);
        for line in s.split_terminator('\n') {
            let clean = line.replace('\t', "    ");
            if !clean.is_empty() {
                let _ = self.0.send(clean.to_string());
            }
        }
        Ok(buf.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
pub fn initialize_logger() -> UnboundedReceiver<String> {
    let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
    LOGGER_TX.set(tx).unwrap();

    Builder::new()
        .write_style(WriteStyle::Always)
        .format(|buf, record| {
            // 1. 时间戳（深灰）
            let ts = buf.timestamp().to_string().dark_grey();

            // 2. 日志级别（自动彩色+重置）
            let level_style = buf.default_level_style(record.level());
            let lvl = format!(
                "{}{}{}",
                level_style.render(),
                record.level(),
                level_style.render_reset()
            );

            // 3. 模块目标（灰色）
            let tgt = record.target().dark_grey();

            // 4. 取出 raw duration 字符串，并格式化到最多两位小数
            let dur_raw = record
                .key_values()
                .get(Key::from("duration"))
                .map(|v| {
                    let s = format!("{}", v);
                    if let Some(idx) = s.find(|c: char| c.is_alphabetic()) {
                        // 拆出数字和单位
                        let (num, unit) = (&s[..idx], &s[idx..]);
                        if let Ok(val) = num.parse::<f32>() {
                            // 格式化到两位小数
                            return format!("{:.2} {}", val, unit);
                        }
                    }
                    // 回退到原始字符串
                    s
                })
                .unwrap_or_default();

            // 5. 无 duration 就输出 10 个空格，否则右对齐到 10 列再上色
            let dur = if dur_raw.is_empty() {
                " ".repeat(10)
            } else {
                format!("{:>10}", dur_raw).cyan().to_string()
            };

            // 6. 输出两行：第一行前缀，第二行 dur + 空格 + message
            writeln!(buf, "{} {} [{}]\n{} {}", ts, lvl, tgt, dur, record.args())?;

            Ok(())
        })
        .target(env_logger::Target::Pipe(Box::new(TokioPipe(
            LOGGER_TX.get().unwrap().clone(),
        ))))
        .filter(None, log::LevelFilter::Info)
        .filter(Some("rocket"), log::LevelFilter::Warn)
        .init();

    rx
}
pub fn check_ffmpeg_and_ffprobe() {
    for command in &["ffmpeg", "ffprobe"] {
        match Command::new(command).arg("-version").output() {
            Ok(output) if output.status.success() => {
                let version_info = String::from_utf8_lossy(&output.stdout);
                let version_number = version_info
                    .lines()
                    .next()
                    .unwrap_or("Unknown version")
                    .split_whitespace()
                    .nth(2) // Get the third word
                    .unwrap_or("Unknown");
                info!("{} version: {}", command, version_number);
            }
            Ok(_) => {
                error!(
                    "`{}` command was found, but it returned an error. Please ensure it's correctly installed.",
                    command
                );
            }
            Err(_) => {
                error!(
                    "`{}` is not installed or not available in PATH. Please install it before running the application.",
                    command
                );
            }
        }
    }
}

pub fn initialize_folder() {
    std::fs::create_dir_all(PathBuf::from("./db")).unwrap();
    std::fs::create_dir_all(PathBuf::from("./object/imported")).unwrap();
    std::fs::create_dir_all(PathBuf::from("./object/compressed")).unwrap();
    std::fs::create_dir_all(PathBuf::from("./upload")).unwrap();
}

pub fn initialize_file() {
    {
        let db_path = "./db/temp_db.redb";
        if fs::metadata(db_path).is_ok() {
            match fs::remove_file(db_path) {
                Ok(_) => {
                    info!("Clear tree cache");
                }
                Err(_) => {
                    error!("Fail to delete cache data ./db/temp_db.redb")
                }
            }
        }
    }
    {
        let db_path = "./db/cache_db.redb";
        if fs::metadata(db_path).is_ok() {
            match fs::remove_file(db_path) {
                Ok(_) => {
                    info!("Clear query cache");
                }
                Err(_) => {
                    error!("Fail to delete cache data ./db/cache_db.redb")
                }
            }
        }
    }
    {
        let db_path = "./db/expire_db.redb";
        if fs::metadata(db_path).is_ok() {
            match fs::remove_file(db_path) {
                Ok(_) => {
                    info!("Clear expire table");
                }
                Err(_) => {
                    error!("Fail to delete expire table ./db/expire_db.redb")
                }
            }
        }
    }
}
