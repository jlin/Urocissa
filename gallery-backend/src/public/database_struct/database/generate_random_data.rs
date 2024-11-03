use crate::public::database_struct::file_modify::FileModify;
use arrayvec::ArrayString;

use rand::{distributions::Alphanumeric, Rng};

use std::collections::{BTreeMap, HashSet};

use super::definition::DataBase;

impl DataBase {
    pub fn generate_random_data() -> Self {
        let hash: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .filter(|c| c.is_ascii_lowercase() || c.is_ascii_digit())
            .take(64)
            .map(char::from)
            .collect();

        let hash = ArrayString::<64>::from(&hash).unwrap();

        let width = rand::thread_rng().gen_range(300..=600);
        let height = rand::thread_rng().gen_range(300..=600);

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
            album: ArrayString::<64>::new(),
            alias: vec![FileModify {
                file: String::from("/"),
                modified: 0,
                scan_time: 0,
            }],
            pending: false,
        }
    }
}
