use crate::config::Config;
use crate::crypto::Crypto;
use crate::dto::{FindRecordRes, Record, RecordJson, SaveRecordReq, SaveRecordRes};
use crate::error::PastebinError;
use crate::limiter::limit_qps;
use crate::repo::RecordRepo;
use crate::utils::now;

use std::sync::Arc;

use nuclear::body::{BodyError, JsonParser};
use nuclear::error::CatchExt;
use nuclear::functional::{ref_handler, ref_middleware};
use nuclear::prelude::{Handler, Request, Response, Result};
use nuclear::response::Json;
use nuclear::router::{SimpleRouter, SimpleRouterExt};

pub struct App {
    config: Config,
    repo: Arc<RecordRepo>,
    crypto: Crypto,
}

macro_rules! bail {
    ($err:expr) => {
        return Err($err.into())
    };
    ($err:expr, $($log:tt)*) => {{
        tracing::error!($($log)*);
        return Err($err.into())
    }};
}

impl App {
    pub fn new(config: Config) -> Result<Self> {
        let repo = Arc::new(RecordRepo::new(&config)?);
        repo.clone().spawn_updater();

        let crypto = Crypto::new(&config);
        Ok(Self {
            config,
            repo,
            crypto,
        })
    }

    pub fn into_handler(self) -> impl Handler {
        let mut find = ref_handler(Self::find_record).boxed();
        let mut save = ref_handler(Self::save_record).boxed();
        let recover = ref_middleware(Self::recover);

        let mut router = SimpleRouter::new();

        if let Some(ref limiter) = self.config.limiter {
            find = limit_qps(find, limiter.find_qps as u64).boxed();
            save = limit_qps(save, limiter.save_qps as u64).boxed();
        }

        router.at("/records/:key").get(find);
        router.at("/records").post(save);
        router.wrap(recover).with_state(Arc::new(self))
    }

    async fn recover(&self, req: Request, next: &dyn Handler) -> Result<Response> {
        match next.handle(req).await.catch::<PastebinError>() {
            Ok(Ok(res)) => Ok(res),
            Ok(Err(err)) => err.res(),
            Err(err) => {
                tracing::error!(%err);
                Err(err)
            }
        }
    }

    async fn find_record(&self, req: Request) -> Result<Response> {
        let key = {
            let key = req.expect_param("key");
            self.crypto.validate(key).ok_or(PastebinError::BadKey)?
        };

        let (json, view_count) = self
            .repo
            .access(&key)
            .await?
            .ok_or(PastebinError::NotFound)?;

        let res: FindRecordRes = match serde_json::from_str::<Record>(json.0.as_ref()) {
            Ok(record) => FindRecordRes::new(record, view_count),
            Err(json_err) => bail!(PastebinError::JsonError, ?key, %json_err, "FIND"),
        };

        tracing::info!(
            "FIND key = {0}, url = http://{1}/{0} , view_count = {2}",
            key,
            self.config.server.hostname,
            view_count,
        );

        Json::ok(res).into()
    }

    async fn save_record(&self, mut req: Request) -> Result<Response> {
        let save_req = {
            let mut json_parser = JsonParser::default();
            json_parser.length_limit(self.config.security.max_post_size);

            json_parser
                .parse::<SaveRecordReq>(&mut req)
                .await
                .catch::<BodyError>()?
                .map_err(|err| match err {
                    BodyError::LengthLimitExceeded => PastebinError::TooLongContent,
                    BodyError::InvalidFormat { source } => {
                        tracing::error!(json_err=%source, "SAVE");
                        PastebinError::JsonError
                    }
                    BodyError::ContentTypeMismatch => PastebinError::JsonError,
                })?
        };

        if save_req.expiration_seconds > self.config.security.max_expiration_seconds {
            bail!(PastebinError::TooLongExpirations)
        }

        let record: Record = Record {
            title: save_req.title,
            lang: save_req.lang,
            content: save_req.content,
            saving_time_seconds: now(),
            expiration_seconds: save_req.expiration_seconds,
        };

        let record_json = RecordJson(serde_json::to_string(&record).unwrap().into());

        let key_gen = || self.crypto.generate();
        let key = self
            .repo
            .save(key_gen, &record_json, record.expiration_seconds)
            .await?;

        tracing::info!(
            "SAVE key = {0}, url = http://{1}/{0}",
            key,
            self.config.server.hostname
        );

        Json::ok(SaveRecordRes { key }).into()
    }
}
