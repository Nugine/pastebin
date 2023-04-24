#![allow(clippy::mutable_key_type)] // false positive

use crate::config::Config;
use crate::crypto::Key;

use std::collections::HashMap;
use std::ops::Not;
use std::time::Duration;

use anyhow::Result;
use tokio::spawn;
use tokio::sync::Mutex;
use tokio::task::JoinHandle;
use tokio::time::sleep;

pub struct AntiBot {
    watch_task_map: Mutex<HashMap<Key, JoinHandle<()>>>,
}

impl AntiBot {
    pub fn new(config: &Config) -> Result<Option<Self>> {
        if config.security.anti_bot.not() {
            return Ok(None);
        }
        let activate_task_map = Mutex::new(HashMap::new());
        Ok(Some(Self {
            watch_task_map: activate_task_map,
        }))
    }

    pub async fn watch_deactivate(&self, key: &Key, on_fail: impl FnOnce() + Send + 'static) {
        let key = key.clone();

        let mut guard = self.watch_task_map.lock().await;
        let map = &mut *guard;

        let task = spawn(async move {
            sleep(Duration::from_secs(2)).await;
            on_fail();
        });

        map.insert(key, task);
    }

    pub async fn cancel_deactivate(&self, key: &Key) {
        let mut guard = self.watch_task_map.lock().await;
        let map = &mut *guard;

        if let Some(task) = map.remove(key) {
            task.abort();
        }
    }
}
