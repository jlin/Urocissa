use serde::{Deserialize, Serialize};
use std::{fs::File, io, path::PathBuf, sync::LazyLock};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "camelCase")]
pub struct PublicConfig {
    pub read_only_mode: bool,
    pub disable_img: bool,
}

pub static PUBLIC_CONFIG: LazyLock<PublicConfig> = LazyLock::new(|| {
    // Attempt to open the config.json file
    let file = File::open("config.json");

    match file {
        Ok(file) => {
            // If the file opens successfully, attempt to deserialize it
            serde_json::from_reader(file).expect("Failed to parse config.json")
        }
        Err(e) => {
            if e.kind() == io::ErrorKind::NotFound {
                info!("config.json not found. Using default configuration.");
                PublicConfig::default()
            } else {
                // For other errors, panic and provide the error message
                panic!("Failed to open config.json: {}", e);
            }
        }
    }
});

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct PrivateConfig {
    pub password: String,
    pub sync_path: Vec<PathBuf>,
    pub discord_hook_url: Option<String>,
}

pub static PRIVATE_CONFIG: LazyLock<PrivateConfig> =
    LazyLock::new(|| serde_json::from_reader(File::open("config.json").unwrap()).unwrap());
