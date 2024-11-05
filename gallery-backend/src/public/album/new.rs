use std::{
    collections::{HashMap, HashSet},
    time::{SystemTime, UNIX_EPOCH},
};

use arrayvec::ArrayString;

use super::Album;

impl Album {
    pub fn new(id: ArrayString<64>, title: Option<String>) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        Self {
            id: id,
            title: title,
            created_time: timestamp,
            cover: None,
            user_defined_metadata: HashMap::new(),
            share_list: vec![],
            tag: HashSet::new(),
            width: 300,
            height: 300,
        }
    }
}
