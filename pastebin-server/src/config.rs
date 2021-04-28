use std::fs;

use anyhow::Result;
use camino::Utf8Path;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub server: Server,
    pub security: Security,
    pub redis: Redis,
    pub cache: Option<Cache>,
    pub limiter: Option<Limiter>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Server {
    pub addr: String,
    pub hostname: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Security {
    pub crypto_key: String,
    pub max_post_size: usize,
    pub max_expiration_seconds: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Redis {
    pub url: String,
    pub key_prefix: String,
    pub max_open_connections: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Cache {
    pub update_duration_seconds: u32,
    pub capacity: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Limiter {
    pub find_qps: u32,
    pub save_qps: u32,
}

impl Config {
    pub fn from_toml(path: &Utf8Path) -> Result<Config> {
        let content = fs::read_to_string(path)?;
        let config = toml::from_str(&content)?;
        Ok(config)
    }

    pub fn default_path() -> &'static Utf8Path {
        "pastebin-server.toml".as_ref()
    }
}
