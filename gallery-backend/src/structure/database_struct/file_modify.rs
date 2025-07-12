use bitcode::{Decode, Encode};
use chrono::Utc;
use serde::{Deserialize, Serialize};

use std::path::{Path, PathBuf};

#[derive(Debug, Default, Clone, Deserialize, Serialize, Decode, Encode, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct FileModify {
    pub file: String,
    pub modified: u128,
    pub scan_time: u128,
}
#[derive(Debug, Default, Clone, Deserialize, Serialize, Decode, Encode, PartialEq, Eq, Hash)]
pub struct FileModifySize {
    pub file_modify: FileModify,
    pub size: u64,
}

impl FileModify {
    pub fn new(file: &Path, modified: u128) -> Self {
        Self {
            file: file.to_string_lossy().into_owned(),
            modified,
            scan_time: Utc::now().timestamp_millis() as u128,
        }
    }
}
