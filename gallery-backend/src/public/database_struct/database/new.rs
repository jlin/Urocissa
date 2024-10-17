use crate::public::{constant::VALID_IMAGE_EXTENSIONS, database_struct::file_modify::FileModify};

use super::definition::DataBase;
use arrayvec::ArrayString;
use std::collections::{BTreeMap, HashSet};

impl DataBase {
    pub fn new(
        hash: ArrayString<64>,
        size: u64,
        ext: String,
        exif_vec: BTreeMap<String, String>,
        alias: Vec<FileModify>,
    ) -> Self {
        Self {
            hash,
            size,
            width: 0,
            height: 0,
            thumbhash: Vec::<u8>::new(),
            phash: Vec::<u8>::new(),
            ext_type: Self::determine_type(&ext),
            ext,
            exif_vec,
            tag: HashSet::<String>::new(),
            album: HashSet::<String>::new(),
            alias,
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
