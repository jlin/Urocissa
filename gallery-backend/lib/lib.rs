use arrayvec::ArrayString;
use bitcode::{Decode, Encode};
use redb::TableDefinition;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::collections::{BTreeMap, HashSet};

use redb::{TypeName, Value};

impl Value for DataBase {
    type SelfType<'a>
        = Self
    where
        Self: 'a;
    type AsBytes<'a>
        = Vec<u8>
    where
        Self: 'a;

    fn fixed_width() -> Option<usize> {
        None
    }
    fn from_bytes<'a>(data: &'a [u8]) -> Self::SelfType<'a>
    where
        Self: 'a,
    {
        bitcode::decode::<Self>(data).expect("Failed to deserialize DataBase")
    }

    fn as_bytes<'a, 'b: 'a>(value: &'a Self::SelfType<'b>) -> Self::AsBytes<'a> {
        bitcode::encode(value)
    }

    fn type_name() -> TypeName {
        TypeName::new("DataBase")
    }
}

impl Value for Album {
    type SelfType<'a>
        = Self
    where
        Self: 'a;
    type AsBytes<'a>
        = Vec<u8>
    where
        Self: 'a;

    fn fixed_width() -> Option<usize> {
        None
    }
    fn from_bytes<'a>(data: &'a [u8]) -> Self::SelfType<'a>
    where
        Self: 'a,
    {
        bitcode::decode::<Self>(data).expect("Failed to deserialize Album")
    }

    fn as_bytes<'a, 'b: 'a>(value: &'a Self::SelfType<'b>) -> Self::AsBytes<'a> {
        bitcode::encode(value)
    }

    fn type_name() -> TypeName {
        TypeName::new("Album")
    }
}

pub const DATA_TABLE: TableDefinition<&str, DataBase> = TableDefinition::new("data");

// Typo in table name "albumm".
pub const ALBUM_TABLE: TableDefinition<&str, Album> = TableDefinition::new("albumm");

#[derive(Debug, Default, Clone, Deserialize, Serialize, Decode, Encode, PartialEq, Eq, Hash)]
pub struct FileModify {
    pub file: String,
    pub modified: u128,
    pub scan_time: u128,
}

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
    pub album: HashSet<ArrayString<64>>,
    pub alias: Vec<FileModify>,
    pub ext_type: String,
    pub pending: bool,
}

#[derive(Debug, Clone, Deserialize, Default, Serialize, Decode, Encode, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
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
#[serde(rename_all = "camelCase")]
pub struct Album {
    pub id: ArrayString<64>,
    pub title: Option<String>,
    pub created_time: u128,
    pub start_time: Option<u128>,
    pub end_time: Option<u128>,
    pub last_modified_time: u128,
    pub cover: Option<ArrayString<64>>,
    pub user_defined_metadata: HashMap<String, Vec<String>>,
    pub share_list: Vec<Share>,
    pub tag: HashSet<String>,
    pub width: u32,
    pub height: u32,
    pub item_count: usize,
    pub item_size: u64,
    pub pending: bool,
}
