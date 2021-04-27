use std::time::{SystemTime, UNIX_EPOCH};

pub fn now() -> u64 {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Err(_) => 0,
        Ok(d) => d.as_secs(),
    }
}
