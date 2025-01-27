use arrayvec::ArrayString;
use bitcode::{Decode, Encode};
use redb::TableDefinition;
use serde::{Deserialize, Serialize};

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

pub const DATA_TABLE: TableDefinition<&str, DataBase> = TableDefinition::new("data");

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
