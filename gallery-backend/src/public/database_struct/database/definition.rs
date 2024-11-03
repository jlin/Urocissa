use crate::public::database_struct::file_modify::FileModify;
use arrayvec::ArrayString;
use bitcode::{Decode, Encode};
use serde::{Deserialize, Serialize};

use std::collections::{BTreeMap, HashSet};

#[derive(Debug, Clone, Deserialize, Default, Serialize, Decode, Encode, PartialEq, Eq)]
pub struct DataBase {
    pub hash: ArrayString<64>,
    pub size: u64,
    pub width: u32,
    pub height: u32,
    pub thumbhash: Vec<u8>,
    pub phash: Vec<u8>,
    pub ext: String,
    pub exif_vec: BTreeMap<String, String>,
    pub tag: HashSet<String>,
    pub album: HashSet<String>,
    pub alias: Vec<FileModify>,
    pub ext_type: String,
    pub pending: bool,
}
