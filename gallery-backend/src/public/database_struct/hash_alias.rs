use std::path::{Path, PathBuf};

use arrayvec::ArrayString;
use bitcode::{Decode, Encode};
use serde::{Deserialize, Serialize};

use super::file_modify::FileModify;

#[derive(Debug, Clone, Deserialize, Serialize, Decode, Encode, PartialEq, Eq, Hash)]
pub struct Alias {
    pub alias: Vec<FileModify>,
}

impl Alias {
    pub fn new(vec_of_file_modify: Vec<FileModify>) -> Self {
        Self {
            alias: vec_of_file_modify,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Decode, Encode, PartialEq, Eq, Hash)]
pub struct AliasSize {
    pub alias: Alias,
    pub size: u64,
}

impl AliasSize {
    pub fn new(alias: Alias, size: u64) -> Self {
        Self { alias, size }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Decode, Encode, PartialEq, Eq, Hash)]
pub struct HashAlias {
    pub hash: ArrayString<64>,
    pub alias: Alias,
}

impl HashAlias {
    pub fn new(hash: ArrayString<64>, alias: Alias) -> Self {
        Self { hash, alias }
    }
    pub fn source_path(&self) -> PathBuf {
        PathBuf::from(&self.alias.alias[0].file)
    }
    pub fn ext(&self) -> String {
        Path::new(&self.source_path())
            .extension()
            .and_then(std::ffi::OsStr::to_str)
            .unwrap_or("")
            .to_string()
            .to_lowercase()
    }

    pub fn imported_path(&self) -> PathBuf {
        PathBuf::from(format!(
            "./object/imported/{}/{}.{}",
            &self.hash[0..2],
            self.hash,
            self.ext()
        ))
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Decode, Encode, PartialEq, Eq, Hash)]
pub struct HashAliasSize {
    pub hash_alias: HashAlias,
    pub size: u64,
}

impl HashAliasSize {
    pub fn new(hash_alias: HashAlias, size: u64) -> Self {
        Self { hash_alias, size }
    }
}
