use anyhow::Error;

use serde_json::json;

use crate::public::config::PRIVATE_CONFIG;

pub fn handle_error(error: Error) -> Error {
    error!("{:?}", error);
    if let Some(url) = &PRIVATE_CONFIG.discord_hook_url {
        send_discord_webhook(url, &error);
    }
    error
}
fn send_discord_webhook(webhook_url: &str, error: &Error) -> () {
    let client = reqwest::blocking::Client::new();
    let debug_string = format!("```rust\n{:?}\n```", error);
    let params = json!({ "content": debug_string });
    client.post(webhook_url).json(&params).send().unwrap();
}
