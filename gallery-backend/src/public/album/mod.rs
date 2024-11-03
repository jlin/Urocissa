use std::collections::HashMap;

use arrayvec::ArrayString;
use bitcode::{Decode, Encode};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Default, Serialize, Decode, Encode, PartialEq, Eq)]
pub struct Share {
    pub id: ArrayString<64>,
    pub title: String,
    pub password: Option<String>,
    pub show_metadata: bool,
    pub show_download: bool,
    pub show_upload: bool,
    pub exp: u64,
}

#[derive(Debug, Clone, Deserialize, Default, Serialize, Decode, Encode, PartialEq, Eq)]
pub struct Album {
    pub id: ArrayString<64>,
    pub title: String,
    pub created_time: u128,
    pub cover: ArrayString<64>,
    pub user_defined_metadata: HashMap<String, Vec<String>>,
}
