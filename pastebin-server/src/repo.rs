use crate::crypto::Key;
use crate::error::RecordError;
use crate::record::RecordJson;
use mobc_redis::{
    redis::{self, AsyncCommands},
    Connection, RedisConnectionManager,
};

pub struct RecordRepo {
    key_prefix: String,
    pool: mobc::Pool<RedisConnectionManager>,
}

const VIEW_COUNT_FIELD: &str = "view_count";
const JSON_FIELD: &str = "json";

impl RecordRepo {
    pub fn new(redis_url: &str, key_prefix: &str, max_open: u64) -> anyhow::Result<Self> {
        let client = redis::Client::open(redis_url)?;
        let manager = RedisConnectionManager::new(client);
        let pool = mobc::Pool::builder().max_open(max_open).build(manager);
        let key_prefix = key_prefix.to_owned();
        Ok(Self { key_prefix, pool })
    }
}

macro_rules! log_redis_err {
    ($redis_key:expr, $ret:expr) => {{
        match $ret {
            Ok(t) => t,
            Err(e) => {
                log::error!("redis_key = {}, redis_error = {}, ", $redis_key, e);
                return Err(RecordError::RedisError);
            }
        }
    }};
}

impl RecordRepo {
    async fn get_conn(&self) -> Result<mobc::Connection<RedisConnectionManager>, RecordError> {
        self.pool.get().await.map_err(|e| {
            log::error!("conn_error = {}", e);
            RecordError::RedisError
        })
    }

    fn concat_key(&self, key: &Key) -> String {
        format!("{}_{}", self.key_prefix, key.0)
    }

    pub async fn save<'a, 's>(
        &'s self,
        key_gen: impl Fn() -> Key,
        json: &RecordJson,
        expiration_seconds: u64,
    ) -> Result<Key, RecordError> {
        let mut conn = self.get_conn().await?;

        loop {
            let key = key_gen();

            let redis_key = self.concat_key(&key);

            let ret = conn.exists(&redis_key).await;
            let exists: bool = log_redis_err!(&redis_key, ret);

            if exists {
                continue;
            }

            let ret = redis::pipe()
                .atomic()
                .hset(&redis_key, VIEW_COUNT_FIELD, 0_u64)
                .hset(&redis_key, JSON_FIELD, &json.0)
                .expire(&redis_key, expiration_seconds as usize)
                .query_async::<Connection, ()>(&mut conn)
                .await;

            if let Err(e) = ret {
                log::error!("redis_key = {}, redis_error = {}, ", &redis_key, e);
                return Err(RecordError::RedisError);
            }

            break Ok(key);
        }
    }

    pub async fn access(&self, key: &Key) -> Result<Option<(RecordJson, u64)>, RecordError> {
        let mut conn = self.get_conn().await?;
        let redis_key = self.concat_key(key);

        let ret = conn.exists(&redis_key).await;
        let exists: bool = log_redis_err!(&redis_key, ret);

        if !exists {
            return Ok(None);
        }

        let ret = redis::pipe()
            .atomic()
            .hincr(&redis_key, VIEW_COUNT_FIELD, 1_u64)
            .hget(&redis_key, JSON_FIELD)
            .query_async::<Connection, (u64, String)>(&mut conn)
            .await;

        let (view, json) = log_redis_err!(&redis_key, ret);

        Ok(Some((RecordJson(json), view)))
    }

    pub async fn incr_view(&self, key: &Key, delta: u64) -> Result<Option<u64>, RecordError> {
        let mut conn = self.get_conn().await?;
        let redis_key = self.concat_key(key);

        let ret = conn.exists(&redis_key).await;
        let exists: bool = log_redis_err!(&redis_key, ret);

        if !exists {
            return Ok(None);
        }

        let ret = conn.hincr(&redis_key, VIEW_COUNT_FIELD, delta).await;

        let count: u64 = log_redis_err!(&redis_key, ret);

        Ok(Some(count))
    }
}

use nuclear::re_exports::async_trait;
use nuclear::{Injector, Provider, ProviderOutput};

pub struct RecordRepoProvider;

#[async_trait]
impl Provider for RecordRepoProvider {
    async fn resolve(&self, injector: &mut Injector) -> ProviderOutput {
        let config = injector.inject_ref::<crate::config::Config>()?;
        let redis = &config.redis;
        let ret = RecordRepo::new(&redis.url, &redis.key_prefix, redis.max_open_connections);

        match ret {
            Ok(repo) => {
                injector.provide(repo);
                Some(Ok(()))
            }
            Err(e) => {
                log::error!("Failed to connect redis: {:?}", &redis.url);
                Some(Err(e.into()))
            }
        }
    }
}
