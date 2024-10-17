use std::{panic::Location, path::PathBuf};

use arrayvec::ArrayString;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::public::config::PRIVATE_CONFIG;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct JsonData(pub &'static str, pub &'static std::path::Path);
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ErrorData<'a> {
    pub error: String,
    pub description: String,
    pub hash: Option<ArrayString<64>>,
    pub file: Option<PathBuf>,
    pub time: u128,
    pub location: &'a Location<'a>,
}

impl<'a> ErrorData<'a> {
    pub fn new(
        error: String,
        description: String,
        hash: Option<ArrayString<64>>,
        file: Option<PathBuf>,
        location: &'a Location,
    ) -> Self {
        ErrorData {
            error,
            description,
            time: Utc::now().timestamp_millis() as u128,
            location,
            hash,
            file,
        }
    }
    pub fn to_formatted_string(&self) -> String {
        format!(
            "**Error**: {}\n**Description**: {}\n**File**: {}\n**Location**: {}\n**Time**: {}",
            self.error,
            self.description,
            self.file
                .as_deref()
                .unwrap_or_else(|| std::path::Path::new("None"))
                .display(),
            self.location,
            self.time,
        )
    }
}

pub fn handle_error(error_data: ErrorData) -> () {
    println!("{:?}", error_data);
    if let Some(url) = &PRIVATE_CONFIG.discord_hook_url {
        send_discord_webhook(url, error_data);
    }
}
fn send_discord_webhook(webhook_url: &str, error_data: ErrorData) -> () {
    let client = reqwest::blocking::Client::new();
    let debug_string = error_data.to_formatted_string();
    let params = json!({ "content": debug_string });
    let result = client.post(webhook_url).json(&params).send();
    if let Err(e) = result {
        println!("Error sending webhook: {:?}", e);
    }
}
