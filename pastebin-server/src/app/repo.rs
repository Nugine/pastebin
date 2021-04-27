use super::config::Config;
use super::crypto::Key;
use super::dto::RecordJson;
use super::error::PastebinError;

use mobc_redis::{mobc, redis, RedisConnectionManager};
use redis::aio::Connection;
use redis::AsyncCommands;

use anyhow::Result;

pub struct RecordRepo {
    key_prefix: Box<str>,
    pool: mobc::Pool<RedisConnectionManager>,
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
    pub fn new(config: &Config) -> Result<Self> {
        let cfg = &config.redis;
        let client = redis::Client::open(&*cfg.url)?;
        let manager = RedisConnectionManager::new(client);
        let pool = mobc::Pool::builder()
            .max_open(cfg.max_open_connections)
            .build(manager);
        let key_prefix = cfg.key_prefix.clone().into_boxed_str();
        Ok(Self { key_prefix, pool })
    }

    const VIEW_COUNT_FIELD: &'static str = "view_count";
    const JSON_FIELD: &'static str = "json";

    async fn get_conn(&self) -> Result<mobc::Connection<RedisConnectionManager>> {
        self.pool.get().await.map_err(|conn_error| {
            tracing::error!(%conn_error);
            anyhow::Error::new(PastebinError::RedisError).context(conn_error)
        })
    }

    fn concat_key(&self, key: &Key) -> String {
        format!("{}_{}", self.key_prefix, key)
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

        Ok(key)
    }

    pub async fn access(&self, key: &Key) -> Result<Option<(RecordJson, u64)>> {
        let mut conn = self.get_conn().await?;
        let redis_key = self.concat_key(key);

        if !exists(&mut conn, &redis_key).await? {
            return Ok(None);
        }

        let ret = redis::pipe()
            .atomic()
            .hincr(&redis_key, Self::VIEW_COUNT_FIELD, 1_u64)
            .hget(&redis_key, Self::JSON_FIELD)
            .query_async::<Connection, (u64, String)>(&mut conn)
            .await;

        let (view, json) = redis_try!(&redis_key, ret);

        Ok(Some((RecordJson(json.into()), view)))
    }

    pub async fn incr_view(&self, key: &Key, delta: u64) -> Result<Option<u64>> {
        let mut conn = self.get_conn().await?;
        let redis_key = self.concat_key(key);

        if !exists(&mut conn, &redis_key).await? {
            return Ok(None);
        }

        let ret = conn.hincr(&redis_key, Self::VIEW_COUNT_FIELD, delta).await;

        let count: u64 = redis_try!(&redis_key, ret);

        Ok(Some(count))
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
