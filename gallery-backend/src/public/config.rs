use serde::{Deserialize, Serialize};
use std::{fs::File, path::PathBuf, sync::LazyLock};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct PrivateConfig {
    pub password: String,
    pub sync_path: Vec<PathBuf>,
    pub discord_hook_url: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct PublicConfig {
    pub read_only_mode: bool,
    pub disable_img: bool,
}

pub static PRIVATE_CONFIG: LazyLock<PrivateConfig> =
    LazyLock::new(|| serde_json::from_reader(File::open("config.json").unwrap()).unwrap());

pub static PUBLIC_CONFIG: LazyLock<PublicConfig> =
    LazyLock::new(|| serde_json::from_reader(File::open("config.json").unwrap()).unwrap());
