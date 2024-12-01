use std::time::{SystemTime, UNIX_EPOCH};

use redb::{ReadableTable, ReadableTableMetadata};

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
            let expired_result = get_current_timestamp_u64() > expire_time;
            if expired_result {
                let expire_write_txn = EXPIRE.in_disk.begin_write().unwrap();
                {
                    let mut expire_write_table = expire_write_txn
                        .open_table(EXPIRE_TABLE_DEFINITIONF)
                        .unwrap();
                    println!("{:?}", expire_write_table);
                    expire_table.iter().unwrap().for_each(|result| {
                        let (key, _) = result.unwrap();
                        let timestamp_in_table = key.value();
                        if timestamp_in_table <= timestamp {
                            expire_write_table.remove(timestamp_in_table).unwrap();
                            info!("Delete expire key: {:?}", timestamp_in_table);
                        }
                    });
                    info!(
                        "{} items remaining in expire table",
                        expire_write_table.len().unwrap()
                    );
                }
                expire_write_txn.commit().unwrap();
            }
            expired_result
        } else {
            false
        }
    } else {
        true
    }
}
