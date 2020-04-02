#![forbid(unsafe_code)]

// stable
mod args;

// stable
mod cache;

// stable
mod config;

// stable
mod crypto;

// todo
mod endpoint;

// todo
mod error;

// todo
mod limiter;

// stable
mod record;

// stable
mod repo;

mod util {
    use std::time::{SystemTime, UNIX_EPOCH};

    pub fn now() -> u64 {
        match SystemTime::now().duration_since(UNIX_EPOCH) {
            Err(_) => 0,
            Ok(d) => d.as_secs(),
        }
    }
}

use anyhow::Context;

use nuclear::*;

use crate::limiter::TokenBucket;
use std::time::Duration;

fn record_module(module: &mut Module, config: &crate::config::Config) {
    let mut find = DynEndpoint::new(crate::endpoint::find_record);
    let mut save = DynEndpoint::new(crate::endpoint::save_record);

    if let Some(ref limiter) = config.limiter {
        let find_limiter = TokenBucket::new(
            Duration::from_secs(1),
            limiter.find_qps,
            limiter.find_qps.saturating_mul(5),
        );

        let save_limiter = TokenBucket::new(
            Duration::from_secs(1),
            limiter.save_qps,
            limiter.save_qps.saturating_mul(5),
        );
        find = find.middleware(find_limiter);
        save = save.middleware(save_limiter);
    }

    module.get("/record/:key", find);
    module.post("/record", save);
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let args = crate::args::get_args();

    {
        let mut builder = env_logger::Builder::from_default_env();
        if let Some(ref log) = args.log {
            builder.parse_filters(log);
        }
        builder.init();
    }

    let config;

    {
        let config_path = match args.config_path {
            Some(ref p) => &*p,
            None => crate::config::default_path(),
        };

        config = crate::config::read_config(config_path)
            .with_context(|| format!("Failed to read config from {:?}", config_path))?;

        log::info!("config_path = {}", config_path.display());
        log::info!("config:\n{:#?}", config);
    }

    let addr = &config.server.addr.clone();

    let app = {
        let mut app = App::new();

        let root: &mut Module = app.root_mut();
        record_module(root, &config);

        app.provide(config)
            .provide_fn(crate::crypto::CryptoProvider)
            .provide_fn(crate::repo::RecordRepoProvider)
            .provide_fn(crate::cache::RecordCacheProvider);

        app
    };

    let server =
        Server::listen(addr).with_context(|| format!("Failed to listen address: {}", &addr))?;

    log::info!("listening on http://{}", addr);
    server.run(app).await
}
