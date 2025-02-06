use crate::public::database_struct::file_modify::FileModify;
use arrayvec::ArrayString;

use rand::{distr::Alphanumeric, Rng};

use std::collections::{BTreeMap, HashSet};

use super::definition::Database;

impl Database {
    pub fn generate_random_data() -> Self {
        let hash: String = rand::rng()
            .sample_iter(&Alphanumeric)
            .filter(|c| c.is_ascii_lowercase() || c.is_ascii_digit())
            .take(64)
            .map(char::from)
            .collect();

        let hash = ArrayString::<64>::from(&hash).unwrap();

        let width = rand::rng().random_range(300..=600);
        let height = rand::rng().random_range(300..=600);

        Self {
            size: 0,
            hash,
            width,
            height,
            thumbhash: Vec::<u8>::new(),
            phash: Vec::<u8>::new(),
            ext_type: "image".to_string(),
            ext: "jpg".to_string(),
            exif_vec: BTreeMap::<String, String>::new(),
            tag: HashSet::<String>::new(),
            album: HashSet::new(),
            alias: vec![FileModify {
                file: String::from("/"),
                modified: 0,
                scan_time: 0,
            }],
            pending: false,
        }
    }
}
