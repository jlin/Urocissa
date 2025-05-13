use crate::{
    constant::VALID_IMAGE_EXTENSIONS, structure::database_struct::file_modify::FileModify,
};

use super::definition::Database;
use arrayvec::ArrayString;
use std::collections::{BTreeMap, HashSet};

impl Database {
    pub fn new(size: u64, hash_alias: FileModify) -> Self {
        let ext = hash_alias.ext();
        Self {
            hash: ArrayString::<64>::default(),
            size,
            width: 0,
            height: 0,
            thumbhash: Vec::<u8>::new(),
            phash: Vec::<u8>::new(),
            ext_type: Self::determine_type(&ext),
            ext,
            exif_vec: BTreeMap::new(),
            tag: HashSet::<String>::new(),
            album: HashSet::new(),
            alias: vec![hash_alias],
            pending: false,
        }
    }
    fn determine_type(ext: &str) -> String {
        if VALID_IMAGE_EXTENSIONS.contains(&ext) {
            "image".to_string()
        } else {
            "video".to_string()
        }
    }
}
