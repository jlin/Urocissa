use std::time::{SystemTime, UNIX_EPOCH};

use super::query_snapshot::{EXPIRE_TABLE_DEFINITIONF, QUERY_SNAPSHOT};

pub fn get_current_timestamp_u64() -> u64 {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    timestamp as u64
}

pub fn expired(timestamp: u64) -> bool {
    let query_read_txn = QUERY_SNAPSHOT.in_disk.begin_read().unwrap();
    let expire_table = query_read_txn.open_table(EXPIRE_TABLE_DEFINITIONF).unwrap();
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
