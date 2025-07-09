use crate::looper::expire::EXPIRE_TABLE_DEFINITION;
use crate::looper::tree::VERSION_COUNT_TIMESTAMP;
use crate::utils::get_current_timestamp_u64;

use log::info;
use std::sync::atomic::Ordering;
use std::time::{Duration, Instant};

use super::{EXPIRE, Expire};

impl Expire {
    pub fn update_expire_time(&self, start_time: Instant) {
        let current_timestamp = get_current_timestamp_u64();
        let last_timestamp = VERSION_COUNT_TIMESTAMP.swap(current_timestamp, Ordering::SeqCst);
        let duration = format!("{:?}", start_time.elapsed());
        info!(duration = &*duration; "In-memory cache updated ({}).", current_timestamp);

        if last_timestamp > 0 {
            let expire_write_txn = self.in_disk.begin_write().unwrap();
            let new_expire_time =
                current_timestamp.saturating_add(Duration::from_secs(60 * 60).as_millis() as u64);

            {
                let mut expire_table = expire_write_txn
                    .open_table(EXPIRE_TABLE_DEFINITION)
                    .expect("Failed to open expire table");

                expire_table
                    .insert(last_timestamp, Some(new_expire_time))
                    .expect("Failed to insert into expire table");
                expire_table
                    .insert(current_timestamp, None)
                    .expect("Failed to insert into expire table");

                info!(
                    "Expire table updated. Next expire time set to {}",
                    new_expire_time
                );
            }

            expire_write_txn.commit().unwrap();
            EXPIRE.expire_check();
        }
    }
}
