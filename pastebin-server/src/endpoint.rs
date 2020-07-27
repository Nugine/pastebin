use crate::cache::RecordCache;
use crate::config::Config;
use crate::crypto::Crypto;
use crate::dto::{FindRecordRes, Record, RecordJson, SaveRecordReq, SaveRecordRes};
use crate::error::RecordError;
use crate::repo::RecordRepo;
use nuclear::http::StatusCode;
use nuclear::{
    core::{InjectorExt, Request, Responder, Response, Result},
    web::{self, body::BodyError, router::SimpleRouterExt},
};
use std::sync::Arc;

// require: Config
// require: Crypto
// require: RecordRepo
// require: RecordCache
// pattern: "/records/:key"
pub async fn find_record(req: Request) -> Result<Response> {
    let key = {
        let key = req.param("key").unwrap();
        let crypto = req.inject_ref::<Crypto>().unwrap();
        crypto.validate(key).ok_or_else(|| RecordError::BadKey)?
    };

    let cache = req.inject_ref::<RecordCache>().unwrap();
    let repo = req.inject_ref::<RecordRepo>().unwrap();

    let record_json: Arc<RecordJson>;
    let view_count: u64;
    let cache_hit: bool;

    match cache.access(&key).await {
        Some((json, count)) => {
            cache_hit = true;
            record_json = json;
            view_count = count;
        }
        None => {
            cache_hit = false;

            let start_time = std::time::Instant::now();

            match repo.access(&key).await? {
                Some((json, count)) => {
                    record_json = Arc::new(json);
                    view_count = count;
                }
                None => {
                    log::warn!(
                        "FIND key = {}, record_error = {}",
                        &key.0,
                        RecordError::NotFound
                    );
                    return Err(RecordError::NotFound.into());
                }
            }

            let time = start_time.elapsed();
            if time.as_secs_f64() > 0.1 {
                log::warn!("FIND slow repo_access: latency = {:?}", time,);
            }
        }
    };

    let log_json_err = |e| {
        log::error!("FIND key = {}, json_error = {}, ", &key.0, e);
        RecordError::JsonError
    };

    let res: FindRecordRes = {
        let record = serde_json::from_str::<Record>(&*record_json.0).map_err(log_json_err)?;
        FindRecordRes::new(record, view_count)
    };

    {
        cache.send_update(
            key.clone(),
            Arc::clone(&record_json),
            res.view_count,
            (res.saving_time_seconds as u64) + (res.expiration_seconds as u64),
        );
    }

    {
        let config = req.inject_ref::<Config>().unwrap();

        log::info!(
            "FIND key = {0}, url = http://{1}/{0} , cache_hit = {2}, view_count = {3}",
            key,
            config.server.hostname,
            cache_hit,
            view_count
        );
    }

    Ok(web::reply::json(res).res().map_err(log_json_err)?)
}

// require: Config
// require: Crypto
// require: RecordRepo
// require: RecordCache
// pattern: "/records"
pub async fn save_record(mut req: Request) -> Result<Response> {
    let save_req: SaveRecordReq = {
        let config = req.inject_ref::<Config>().unwrap();
        let mut json_parser = web::body::JsonParser::default();
        json_parser.limit(config.security.max_post_size);

        match json_parser.parse(&mut req).await {
            Ok(s) => s,
            Err(e) => match e {
                BodyError::Limit(_) => return Err(RecordError::TooLongContent.into()),
                BodyError::Parse(e) => {
                    log::error!("SAVE json error = {}", e);
                    return Err(RecordError::JsonError.into());
                }
                BodyError::ContentTypeMismatch => {
                    let mut res = Response::new("");
                    res.set_status(StatusCode::BAD_REQUEST);
                    return Ok(res);
                }
                e => return Err(e.into()),
            },
        }
    };

    let config = req.inject_ref::<Config>().unwrap();
    if save_req.expiration_seconds > config.security.max_expiration_seconds {
        return Err(RecordError::TooLongExpirations.into());
    }

    let record: Record = Record {
        title: save_req.title,
        lang: save_req.lang,
        content: save_req.content,
        saving_time_seconds: crate::util::now() as u32,
        expiration_seconds: save_req.expiration_seconds,
    };

    let repo = req.inject_ref::<RecordRepo>().unwrap();
    let crypto = req.inject_ref::<Crypto>().unwrap();

    let record_json = Arc::new(RecordJson(serde_json::to_string(&record).unwrap()));

    let key = repo
        .save(
            || crypto.generate(),
            &*record_json,
            record.expiration_seconds,
        )
        .await?;

    {
        let cache = req.inject_ref::<RecordCache>().unwrap();
        let dead_time = (record.saving_time_seconds as u64) + (record.expiration_seconds as u64);
        if cache.is_updating() {
            cache.send_update(key.clone(), record_json, 0, dead_time);
        } else {
            cache
                .force_update(key.clone(), record_json, 0, dead_time)
                .await;
        }
    }

    {
        let config = req.inject_ref::<Config>().unwrap();

        log::info!(
            "SAVE key = {0}, url = http://{1}/{0}",
            key,
            config.server.hostname
        );
    }

    web::reply::json(SaveRecordRes { key }).try_response()
}
