use env_logger::{Builder, WriteStyle};
use log::kv::Key;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use superconsole::style::Stylize;

use std::io::Write;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

use crate::public::tui::LOGGER_TX;

pub mod ffmpeg;
pub mod folder;
pub mod logger;
pub mod redb;
