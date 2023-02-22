use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

/// seconds since the unix epoch
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct UnixTimestamp(u64);

impl UnixTimestamp {
    pub fn now() -> Option<Self> {
        let d = SystemTime::now().duration_since(UNIX_EPOCH).ok()?;
        Some(UnixTimestamp(d.as_secs()))
    }
}
