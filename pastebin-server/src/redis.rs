use crate::config::RedisConfig;
use crate::crypto::Key;
use crate::dto::Record;

use mobc_redis::mobc;
use mobc_redis::redis;
use redis::aio::MultiplexedConnection;
use redis::AsyncCommands;

use anyhow::Result;
use tracing::error;

pub struct RedisStorage {
    key_prefix: Box<str>,
    pool: mobc::Pool<mobc_redis::RedisConnectionManager>,
}

const VIEW_COUNT_FIELD: &str = "view_count";
const JSON_FIELD: &str = "json";

async fn exists(conn: &mut MultiplexedConnection, redis_key: &str) -> Result<bool> {
    let exists: bool = conn
        .exists(redis_key)
        .await
        .inspect_err(|err| error!(?err))?;
    Ok(exists)
}

impl RedisStorage {
    pub fn new(config: &RedisConfig) -> Result<Self> {
        let key_prefix = config.key_prefix.clone().into_boxed_str();

        let pool = {
            let client = redis::Client::open(&*config.url)?;
            let manager = mobc_redis::RedisConnectionManager::new(client);
            mobc::Pool::builder()
                .max_open(config.max_open_connections)
                .build(manager)
        };

        Ok(Self { key_prefix, pool })
    }

    async fn get_conn(&self) -> Result<mobc::Connection<mobc_redis::RedisConnectionManager>> {
        let conn = self.pool.get().await.inspect_err(|err| error!(?err))?;
        Ok(conn)
    }

    fn concat_key(&self, key: &Key) -> String {
        format!("{}:{}", self.key_prefix, key.as_str())
    }

    pub async fn save(
        &self,
        key_gen: impl Fn() -> Key,
        record: &Record,
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

        let json = serde_json::to_string(record).unwrap();

        redis::pipe()
            .atomic()
            .hset(&redis_key, VIEW_COUNT_FIELD, 0_u64)
            .hset(&redis_key, JSON_FIELD, &*json)
            .expire(&redis_key, expiration_seconds as i64)
            .query_async::<()>(&mut *conn)
            .await
            .inspect_err(|err| error!(?err))?;

        Ok(key)
    }

    pub async fn access(&self, key: &Key) -> Result<Option<(Record, u64)>> {
        let mut conn = self.get_conn().await?;
        let redis_key = self.concat_key(key);

        if !exists(&mut conn, &redis_key).await? {
            return Ok(None);
        }

        let (view, json): (u64, String) = redis::pipe()
            .atomic()
            .hincr(&redis_key, VIEW_COUNT_FIELD, 1_u64)
            .hget(&redis_key, JSON_FIELD)
            .query_async(&mut *conn)
            .await
            .inspect_err(|err| error!(?err))?;

        let record: Record = serde_json::from_str(&json).inspect_err(|err| error!(?err))?;
        Ok(Some((record, view)))
    }

    pub async fn delete(&self, key: &Key) -> Result<bool> {
        let mut conn = self.get_conn().await?;
        let redis_key = self.concat_key(key);

        let deleted: bool = conn.del(redis_key).await.inspect_err(|err| error!(?err))?;
        Ok(deleted)
    }
}
