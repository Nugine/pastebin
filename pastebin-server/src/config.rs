use std::fs;

use anyhow::Result;
use camino::Utf8Path;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub server: ServerConfig,
    pub security: SecurityConfig,
    pub redis: RedisConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub bind_addr: String,
    pub host_addr: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    pub secret_key: String,
    pub max_body_length: usize,
    pub max_expiration_seconds: u32,
    pub max_qps: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedisConfig {
    pub url: String,
    pub key_prefix: String,
    pub max_open_connections: u64,
}

impl Config {
    pub fn from_toml(path: &Utf8Path) -> Result<Config> {
        let content = fs::read_to_string(path)?;
        let config = toml::from_str(&content)?;
        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_config() {
        let config_path = concat!(env!("CARGO_MANIFEST_DIR"), "/pastebin-server.toml");
        let config = Config::from_toml(config_path.as_ref()).unwrap();
        println!("{config:#?}");
    }
}
