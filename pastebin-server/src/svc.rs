use crate::anti_bot::AntiBot;
use crate::block_rules::BlockRules;
use crate::config::Config;
use crate::crypto::Crypto;
use crate::crypto::Key;
use crate::dto::*;
use crate::error::PastebinError;
use crate::error::PastebinErrorCode;
use crate::redis::RedisStorage;
use crate::time::UnixTimestamp;

use std::sync::Arc;

use anyhow::Result;
use tokio::spawn;
use tracing::error;
use tracing::info;

pub struct PastebinService {
    config: Config,
    db: RedisStorage,
    crypto: Crypto,

    block_rules: Option<BlockRules>,
    anti_bot: Option<AntiBot>,
}

impl PastebinService {
    pub fn new(config: &Config) -> Result<Self> {
        let block_rules = BlockRules::new(config)?;

        let anti_bot = AntiBot::new(config)?;

        let db = RedisStorage::new(&config.redis)?;

        let crypto = Crypto::new(&config.security.secret_key);

        let config = config.clone();

        Ok(Self {
            config,
            db,
            crypto,
            block_rules,
            anti_bot,
        })
    }

    pub async fn find_record(
        self: &Arc<Self>,
        input: FindRecordInput,
    ) -> Result<FindRecordOutput, PastebinError> {
        let key = self
            .crypto
            .validate(&input.key)
            .ok_or(PastebinErrorCode::BadKey)?;

        if let Some(anti_bot) = self.anti_bot.as_ref() {
            anti_bot.cancel_deactivate(&key).await;
        }

        let result = self.db.access(&key).await;

        let (record, view_count) = result
            .inspect_err(|err| error!(?err))
            .map_err(|_| PastebinErrorCode::InternalError)?
            .ok_or(PastebinErrorCode::NotFound)?;

        info!("FIND key = {}, view_count = {}", key.as_str(), view_count);

        Ok(FindRecordOutput { record, view_count })
    }

    pub async fn save_record(
        self: &Arc<Self>,
        input: SaveRecordInput,
    ) -> Result<SaveRecordOutput, PastebinError> {
        if input.title.chars().count() > self.config.security.max_title_chars {
            return Err(PastebinErrorCode::TooLongTitle.into());
        }

        if input.expiration_seconds > self.config.security.max_expiration_seconds {
            return Err(PastebinErrorCode::TooLongExpirations.into());
        }

        if let Some(block_rules) = self.block_rules.as_ref() {
            if block_rules.is_match(&input) {
                let key = self.crypto.generate();
                info!("BLOCKED key = {}", key.as_str());
                return Ok(SaveRecordOutput { key });
            }
        }

        let now = UnixTimestamp::now().expect("must be after 1970");

        let record = Record {
            title: input.title,
            lang: input.lang,
            content: input.content,
            saving_time: now,
            expiration_seconds: input.expiration_seconds,
        };

        let key_gen = || self.crypto.generate();
        let expiration = record.expiration_seconds;
        let result = self.db.save(key_gen, &record, expiration).await;

        let key = result
            .inspect_err(|err| error!(?err))
            .map_err(|_| PastebinErrorCode::InternalError)?;

        if let Some(anti_bot) = self.anti_bot.as_ref() {
            let on_fail = {
                let svc = self.clone();
                let key = key.clone();
                || deactivate_new_key(svc, key)
            };
            anti_bot.watch_deactivate(&key, on_fail).await;
        }

        info!("SAVE key = {}, expiration = {}", key.as_str(), expiration);

        Ok(SaveRecordOutput { key })
    }
}

fn deactivate_new_key(svc: Arc<PastebinService>, key: Key) {
    drop(spawn(async move {
        let result = svc.db.delete(&key).await;
        match result {
            Ok(true) => info!("ANTIBOT DEL key = {}", key.as_str()),
            Ok(false) => {}
            Err(err) => error!(?err),
        }
    }))
}
