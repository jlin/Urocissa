mod validator_extension;
mod validator_hash;
mod validator_modified;

use arrayvec::ArrayString;
use dashmap::DashMap;
use std::path::PathBuf;

use crate::public::database_struct::{
    database::definition::DataBase,
    hash_alias::{AliasSize, HashAlias, HashAliasSize},
};

pub fn filter(all_paths: Vec<PathBuf>) -> DashMap<ArrayString<64>, DataBase> {
    let valid_extension_paths = validator_extension::validator(all_paths);
    let vec_of_file_modify = validator_modified::validator(valid_extension_paths);
    let vec_of_file_modify_hash = validator_hash::validator(vec_of_file_modify);
    vec_of_file_modify_hash
}
