use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub fn get_current_timestamp_u64() -> u64 {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    timestamp as u64
}

pub fn info_wrap(duration_opt: Option<Duration>, msg: &str) {
    if let Some(duration_inner) = duration_opt {
        info!(duration = &*format!("{:?}", duration_inner); "{}", msg);
    } else {
        info!("{}", msg);
    }
}

pub fn warn_wrap(duration_opt: Option<Duration>, msg: &str) {
    if let Some(duration_inner) = duration_opt {
        warn!(duration = &*format!("{:?}", duration_inner); "{}", msg);
    } else {
        warn!("{}", msg);
    }
}
