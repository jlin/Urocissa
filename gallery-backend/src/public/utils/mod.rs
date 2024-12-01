use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub fn get_current_timestamp_u64() -> u64 {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    timestamp as u64
}

pub fn expired(timestamp: u64) -> bool {
    get_current_timestamp_u64() > timestamp + (Duration::from_secs(1).as_millis() as u64)
    // 1 hour
}
