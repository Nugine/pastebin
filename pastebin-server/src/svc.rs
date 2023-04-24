use crate::block::BlockRules;
use crate::config::Config;
use crate::crypto::Crypto;
use crate::dto::*;
use crate::error::PastebinError;
use crate::error::PastebinErrorCode;
use crate::redis::RedisStorage;
use crate::time::UnixTimestamp;

use anyhow::Result;
use tracing::error;

pub struct PastebinService {
    config: Config,
    db: RedisStorage,
    crypto: Crypto,

    block_rules: Option<BlockRules>,
}

impl PastebinService {
    pub fn new(config: &Config) -> Result<Self> {
        let block_rules = BlockRules::new(config)?;

        let db = RedisStorage::new(&config.redis)?;

        let crypto = Crypto::new(&config.security.secret_key);

        let config = config.clone();

        Ok(Self {
            config,
            db,
            crypto,
            block_rules,
        })
    }

    pub async fn find_record(
        &self,
        input: FindRecordInput,
    ) -> Result<FindRecordOutput, PastebinError> {
        let key = self
            .crypto
            .validate(&input.key)
            .ok_or(PastebinErrorCode::BadKey)?;

        let result = self.db.access(&key).await;

        let (record, view_count) = result
            .inspect_err(|err| error!(?err))
            .map_err(|_| PastebinErrorCode::InternalError)?
            .ok_or(PastebinErrorCode::NotFound)?;

        tracing::info!(
            "FIND key = {0}, url = http://{1}/{0} , view_count = {2}",
            key.as_str(),
            self.config.server.host_addr,
            view_count,
        );

        Ok(FindRecordOutput { record, view_count })
    }

    pub async fn save_record(
        &self,
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
                tracing::info!("BLOCKED key = {}", key.as_str());
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

        tracing::info!(
            "SAVE key = {0}, url = http://{1}/{0} , expiration = {2}",
            key.as_str(),
            self.config.server.host_addr,
            expiration,
        );

        Ok(SaveRecordOutput { key })
    }
}
