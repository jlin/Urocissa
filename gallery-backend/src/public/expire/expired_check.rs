use redb::{ReadableTable, ReadableTableMetadata};

use crate::public::utils::get_current_timestamp_u64;

use super::{Expire, EXPIRE_TABLE_DEFINITION};

impl Expire {
    pub fn expired_check(&self, timestamp: u64) -> bool {
        let expire_read_txn = self.in_disk.begin_read().unwrap();
        let expire_table = expire_read_txn.open_table(EXPIRE_TABLE_DEFINITION).unwrap();
        if let Some(expire_timestamp) = expire_table.get(timestamp).unwrap() {
            if let Some(expire_time) = expire_timestamp.value() {
                let expired_result = get_current_timestamp_u64() > expire_time;
                if expired_result {
                    let expire_write_txn = self.in_disk.begin_write().unwrap();
                    {
                        let mut expire_write_table = expire_write_txn
                            .open_table(EXPIRE_TABLE_DEFINITION)
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
}
