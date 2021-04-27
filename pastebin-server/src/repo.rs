use crate::config::Config;
use crate::crypto::Key;
use crate::dto::RecordJson;
use crate::error::PastebinError;
use crate::utils::now;

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Duration;

use mobc_redis::{mobc, redis, RedisConnectionManager};
use rand::Rng;
use redis::aio::Connection;
use redis::AsyncCommands;

use anyhow::Result;
use dashmap::mapref::entry::Entry;
use dashmap::DashMap;
use tokio::sync::RwLock;
use tokio::task;
use tokio::time::interval;

pub struct RecordRepo {
    key_prefix: Box<str>,
    pool: mobc::Pool<RedisConnectionManager>,
    cache: Option<Cache>,
}

struct Cache {
    map: RwLock<DashMap<Key, CacheItem>>,
    update_duration: Duration,
    capacity: usize,
}

struct CacheItem {
    json: RecordJson,
    view_count: u64,
    delta: AtomicU64,
    dead_time: u64,
}

macro_rules! redis_try {
    ($redis_key:expr, $ret:expr) => {{
        match $ret {
            Ok(t) => t,
            Err(redis_error) => {
                let redis_key = $redis_key;
                tracing::error!(?redis_key, %redis_error);
                let err = anyhow::Error::new(PastebinError::RedisError).context(redis_error);
                return Err(err);
            }
        }
    }};
}

async fn exists(conn: &mut Connection, redis_key: &str) -> Result<bool> {
    let exists: bool = redis_try!(&redis_key, conn.exists(redis_key).await);
    Ok(exists)
}

impl RecordRepo {
    const VIEW_COUNT_FIELD: &'static str = "view_count";
    const JSON_FIELD: &'static str = "json";

    pub fn new(config: &Config) -> Result<Self> {
        let redis = &config.redis;
        let key_prefix = redis.key_prefix.clone().into_boxed_str();
        let pool = {
            let client = redis::Client::open(&*redis.url)?;
            let manager = RedisConnectionManager::new(client);
            mobc::Pool::builder()
                .max_open(redis.max_open_connections)
                .build(manager)
        };

        let cache = config.cache.as_ref().map(|cache| Cache {
            map: RwLock::new(DashMap::new()),
            update_duration: Duration::from_secs(cache.update_duration_seconds as u64),
            capacity: cache.capacity,
        });

        Ok(Self {
            key_prefix,
            pool,
            cache,
        })
    }

    async fn get_conn(&self) -> Result<mobc::Connection<RedisConnectionManager>> {
        self.pool.get().await.map_err(|conn_error| {
            tracing::error!(%conn_error);
            anyhow::Error::new(PastebinError::RedisError).context(conn_error)
        })
    }

    fn concat_key(&self, key: &Key) -> String {
        format!("{}_{}", self.key_prefix, key)
    }

    async fn incr_view(&self, key: &Key, delta: u64) -> Result<Option<u64>> {
        let mut conn = self.get_conn().await?;
        let redis_key = self.concat_key(key);

        if !exists(&mut conn, &redis_key).await? {
            return Ok(None);
        }

        let ret = conn.hincr(&redis_key, Self::VIEW_COUNT_FIELD, delta).await;

        let count: u64 = redis_try!(&redis_key, ret);

        Ok(Some(count))
    }

    async fn updater(self: Arc<Self>) {
        let cache = self.cache.as_ref().unwrap();

        let mut int = interval(cache.update_duration);
        int.tick().await;

        loop {
            int.tick().await;

            let mut guard = cache.map.write().await;
            let map = &mut *guard;
            for mut pair in map.iter_mut() {
                let (k, v) = pair.pair_mut();
                let delta = v.delta.get_mut();
                let d = *delta;
                if d != 0 {
                    let this = Arc::clone(&self);
                    let key = k.clone();
                    task::spawn(async move { this.incr_view(&key, d).await });
                    v.view_count += d;
                    *delta = 0;
                }
            }

            let now = now();
            let mut rng = rand::thread_rng();
            let len = map.len();
            let cap = cache.capacity;
            map.retain(|_, v| v.dead_time > now && rng.gen_range(0..len) < cap)
        }
    }

    pub async fn save(
        &self,
        key_gen: impl Fn() -> Key,
        json: &RecordJson,
        expiration_seconds: u32,
    ) -> Result<Key> {
        let mut conn = self.get_conn().await?;

        let (key, redis_key) = loop {
            let key = key_gen();
            let redis_key = self.concat_key(&key);

            if !exists(&mut conn, &redis_key).await? {
                break (key, redis_key);
            }
        };

        let ret = redis::pipe()
            .atomic()
            .hset(&redis_key, Self::VIEW_COUNT_FIELD, 0_u64)
            .hset(&redis_key, Self::JSON_FIELD, &*json.0)
            .expire(&redis_key, expiration_seconds as usize)
            .query_async::<Connection, ()>(&mut conn)
            .await;

        redis_try!(&redis_key, ret);

        if let Some(cache) = self.cache.as_ref() {
            let dead_time = now() + expiration_seconds as u64;
            cache.update(key.clone(), json.clone(), 0, dead_time).await
        }

        Ok(key)
    }

    pub async fn access(&self, key: &Key) -> Result<Option<(RecordJson, u64)>> {
        if let Some(cache) = self.cache.as_ref() {
            let cache_ret = cache.access(key);
            if cache_ret.is_some() {
                return Ok(cache_ret);
            }
        }

        let mut conn = self.get_conn().await?;
        let redis_key = self.concat_key(key);

        if !exists(&mut conn, &redis_key).await? {
            return Ok(None);
        }

        let ret = redis::pipe()
            .atomic()
            .hincr(&redis_key, Self::VIEW_COUNT_FIELD, 1_u64)
            .hget(&redis_key, Self::JSON_FIELD)
            .ttl(&redis_key)
            .query_async::<Connection, (u64, String, u64)>(&mut conn)
            .await;

        let (view, json, ttl) = redis_try!(&redis_key, ret);
        let json = RecordJson(json.into());

        if let Some(cache) = self.cache.as_ref() {
            let dead_time = now() + ttl;
            cache
                .update(key.clone(), json.clone(), view, dead_time)
                .await
        }

        Ok(Some((json, view)))
    }
}

//         match ret {
//             Ok(repo) => Ok(repo),
//             Err(e) => {
//                 log::error!("Failed to connect redis: {:?}", &redis.url);
//                 Err(e.into())
//             }
//         }
//     }

impl Cache {
    fn access(&self, key: &Key) -> Option<(RecordJson, u64)> {
        let map = self.map.try_read().ok()?;
        let item = map.get(key)?;
        if item.dead_time <= now() {
            return None;
        }
        let delta: u64 = item.delta.fetch_add(1, Ordering::SeqCst) + 1;
        Some((item.json.clone(), item.view_count + delta))
    }

    async fn update(&self, key: Key, json: RecordJson, view_count: u64, dead_time: u64) {
        if dead_time <= now() {
            return;
        }
        let map = self.map.read().await;
        match map.entry(key) {
            Entry::Occupied(mut entry) => {
                let item = entry.get_mut();
                item.json = json;
                item.view_count = item.view_count.max(view_count);
                item.dead_time = item.dead_time.max(dead_time);
            }
            Entry::Vacant(entry) => {
                entry.insert(CacheItem {
                    json,
                    view_count,
                    dead_time,
                    delta: AtomicU64::new(0),
                });
            }
        };
    }
}
