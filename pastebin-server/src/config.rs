use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct MemoryCache {
    pub update_duration_seconds: u64,
    pub capacity: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Redis {
    pub url: String,
    pub key_prefix: String,
    pub max_open_connections: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Limiter {
    pub find_qps: u64,
    pub save_qps: u64,
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
    pub max_expiration_seconds: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub server: Server,
    pub security: Security,
    pub redis: Redis,
    pub memory_cache: Option<MemoryCache>,
    pub limiter: Option<Limiter>,
}

pub fn read_config(path: &Path) -> anyhow::Result<Config> {
    let content = std::fs::read_to_string(path)?;
    let config = toml::from_str(&content)?;
    Ok(config)
}

pub fn default_path() -> &'static Path {
    Path::new("pastebin-server.toml")
}
