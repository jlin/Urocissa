use std::collections::{HashMap, HashSet};

use arrayvec::ArrayString;
use bitcode::{Decode, Encode};
use serde::{Deserialize, Serialize};

pub mod new;

#[derive(Debug, Clone, Deserialize, Default, Serialize, Decode, Encode, PartialEq, Eq)]
pub struct Share {
    pub url: ArrayString<64>,
    pub description: String,
    pub password: Option<String>,
    pub show_metadata: bool,
    pub show_download: bool,
    pub show_upload: bool,
    pub exp: u64,
}

#[derive(Debug, Clone, Deserialize, Default, Serialize, Decode, Encode, PartialEq, Eq)]
pub struct Album {
    pub id: ArrayString<64>,
    pub title: Option<String>,
    pub created_time: u128,
    pub cover: Option<ArrayString<64>>,
    pub user_defined_metadata: HashMap<String, Vec<String>>,
    pub share_list: Vec<Share>,
    pub tag: HashSet<String>,
    pub width: u32,
    pub height: u32,
}
