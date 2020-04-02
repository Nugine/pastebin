use crate::crypto::Key;
use crate::record::RecordJson;
use crate::repo::RecordRepo;
use rand::Rng;
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc::{self, error::TryRecvError};
use tokio::sync::RwLock;

struct UpdateMsg {
    key: Key,
    json: Arc<RecordJson>,
    view_count: u64,
    dead_time: u64,
}

type UpdateTx = mpsc::UnboundedSender<UpdateMsg>;
type UpdateRx = mpsc::UnboundedReceiver<UpdateMsg>;

#[derive(Debug)]
struct CacheItem {
    json: Arc<RecordJson>,
    view_count: u64,
    delta: AtomicU64,
    dead_time: u64,
}

type Cache = HashMap<Key, CacheItem>;

pub struct RecordCache {
    cache: Arc<RwLock<Cache>>,
    update_tx: Option<UpdateTx>,
    updating_flag: Arc<AtomicBool>,
}

impl RecordCache {
    pub fn new() -> Self {
        Self {
            cache: Arc::new(RwLock::new(HashMap::new())),
            update_tx: None,
            updating_flag: Arc::new(false.into()),
        }
    }

    async fn updater(
        mut rx: UpdateRx,
        repo: Arc<RecordRepo>,
        cache: Arc<RwLock<Cache>>,
        duration: Duration,
        updating_flag: Arc<AtomicBool>,
        capacity: usize,
    ) {
        let mut map: Cache = HashMap::new();

        let update_map = |map: &mut Cache, msg: UpdateMsg| {
            let UpdateMsg {
                key,
                json,
                view_count,
                dead_time,
            } = msg;

            map.entry(key)
                .and_modify(|item| item.view_count = item.view_count.max(view_count))
                .or_insert(CacheItem {
                    json,
                    view_count,
                    delta: 0.into(),
                    dead_time,
                });
        };

        loop {
            tokio::time::delay_for(duration).await;

            loop {
                match rx.try_recv() {
                    Ok(msg) => update_map(&mut map, msg),
                    Err(TryRecvError::Closed) => return,
                    Err(TryRecvError::Empty) => break,
                }
            }

            updating_flag.store(true, Ordering::SeqCst);

            let mut lock = cache.write().await;
            let cache: &mut Cache = &mut *lock;

            let mut to_update: Vec<(Key, u64)> = Vec::new();
            let now = crate::util::now();
            let len = cache.len();
            let mut rng = rand::thread_rng();

            cache.retain(|key, item| {
                let delta = *item.delta.get_mut();
                if delta != 0 {
                    to_update.push((key.clone(), delta));
                    *item.delta.get_mut() = 0;
                }
                let idx: usize = rng.gen_range(0, len);
                item.dead_time > now && idx < capacity
            });

            for (key, item) in map.drain() {
                if item.dead_time > now {
                    cache
                        .entry(key)
                        .and_modify(|prev| {
                            let cnt = &mut prev.view_count;
                            *cnt = (*cnt).max(item.view_count);
                        })
                        .or_insert(item);
                }
            }
            drop(lock);

            updating_flag.store(false, Ordering::SeqCst);

            let repo = Arc::clone(&repo);
            tokio::spawn(async move {
                for (key, delta) in to_update {
                    if let Ok(Some(count)) = repo.incr_view(&key, delta).await {
                        log::info!("incr_view: key = {}, view_count = {}", key, count);
                    }
                }
            });
        }
    }

    pub fn spawn_updater(
        &mut self,
        repo: Arc<RecordRepo>,
        duration: Duration,
        cache_capacity: usize,
    ) {
        let (tx, rx) = mpsc::unbounded_channel();
        self.update_tx = Some(tx);
        let cache = Arc::clone(&self.cache);
        let updating_flag = Arc::clone(&self.updating_flag);
        tokio::spawn(Self::updater(
            rx,
            repo,
            cache,
            duration,
            updating_flag,
            cache_capacity,
        ));
    }

    pub async fn access(&self, key: &Key) -> Option<(Arc<RecordJson>, u64)> {
        self.update_tx.as_ref()?;
        let lock = self.cache.read().await;
        let cache: &HashMap<_, _> = &*lock;
        let item = cache.get(key)?;
        let delta: u64 = item.delta.fetch_add(1, Ordering::SeqCst) + 1;
        Some((Arc::clone(&item.json), item.view_count + delta))
    }

    pub fn send_update(&self, key: Key, json: Arc<RecordJson>, view_count: u64, dead_time: u64) {
        if let Some(tx) = self.update_tx.as_ref() {
            let ret = tx.send(UpdateMsg {
                key,
                json,
                view_count,
                dead_time,
            });
            if let Err(e) = ret {
                log::error!("cache_updater terminated: send_error = {}", e);
            }
        }
    }

    pub async fn force_update(
        &self,
        key: Key,
        json: Arc<RecordJson>,
        view_count: u64,
        dead_time: u64,
    ) {
        let mut lock = self.cache.write().await;
        let cache: &mut Cache = &mut *lock;

        let item = CacheItem {
            json,
            view_count,
            delta: 0.into(),
            dead_time,
        };

        log::info!("force_update cache: key = {}", &key.0);

        cache
            .entry(key)
            .and_modify(|item| item.view_count = item.view_count.max(view_count))
            .or_insert(item);
    }

    pub fn is_updating(&self) -> bool {
        self.updating_flag.load(Ordering::SeqCst)
    }
}

use nuclear::re_exports::async_trait;
use nuclear::{Injector, Provider, ProviderOutput};

pub struct RecordCacheProvider;

#[async_trait]
impl Provider for RecordCacheProvider {
    async fn resolve(&self, injector: &mut Injector) -> ProviderOutput {
        let config = injector.inject_ref::<crate::config::Config>()?;
        let repo = injector.inject_arc::<crate::repo::RecordRepo>()?;

        let mut record_cache = RecordCache::new();
        if let Some(mc) = config.memory_cache.as_ref() {
            record_cache.spawn_updater(
                repo,
                Duration::from_secs(mc.update_duration_seconds),
                mc.capacity,
            );
        }

        injector.provide(record_cache);
        Some(Ok(()))
    }
}
