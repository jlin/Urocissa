mod validator_extension;
mod validator_hash;
mod validator_modified;

use arrayvec::ArrayString;
use dashmap::DashMap;
use std::path::PathBuf;

use crate::public::database_struct::hash_alias::{AliasSize, HashAlias, HashAliasSize};

pub fn filter(all_paths: Vec<PathBuf>) -> Vec<HashAliasSize> {
    let valid_extension_paths = validator_extension::validator(all_paths);
    let vec_of_file_modify = validator_modified::validator(valid_extension_paths);
    let vec_of_file_modify_hash = validator_hash::validator(vec_of_file_modify);
    return from_dashmap_hash_alias_to_vec_of_hash_alias(vec_of_file_modify_hash);
}

fn from_dashmap_hash_alias_to_vec_of_hash_alias(
    dashmap_of_hash_alias: DashMap<ArrayString<64>, AliasSize>,
) -> Vec<HashAliasSize> {
    let vec_of_hash_alias_deduplicated = dashmap_of_hash_alias
        .into_iter()
        .map(|(key, value)| HashAliasSize::new(HashAlias::new(key, value.alias), value.size))
        .collect();
    return vec_of_hash_alias_deduplicated;
}
