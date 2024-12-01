use std::time::{SystemTime, UNIX_EPOCH};

use super::expire::{EXPIRE, EXPIRE_TABLE_DEFINITIONF};

pub fn get_current_timestamp_u64() -> u64 {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    timestamp as u64
}

pub fn expired(timestamp: u64) -> bool {
    let expire_read_txn = EXPIRE.in_disk.begin_read().unwrap();
    let expire_table = expire_read_txn
        .open_table(EXPIRE_TABLE_DEFINITIONF)
        .unwrap();
    if let Some(expire_timestamp) = expire_table.get(timestamp).unwrap() {
        if let Some(expire_time) = expire_timestamp.value() {
            get_current_timestamp_u64() > expire_time
        } else {
            false
        }
    } else {
        false
    }
}
