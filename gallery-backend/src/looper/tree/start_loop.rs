use std::sync::atomic::AtomicU64;

pub static VERSION_COUNT_TIMESTAMP: AtomicU64 = AtomicU64::new(0);
