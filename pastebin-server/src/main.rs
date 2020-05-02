#![forbid(unsafe_code)]

mod args;
mod cache;
mod config;
mod crypto;
mod error;
mod limiter;
mod record_endpoint;
mod record_module;
mod record_types;
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

type BoxStdError = Box<dyn std::error::Error + Send + Sync>;

fn get_config(args: &crate::args::Args) -> Result<crate::config::Config, BoxStdError> {
    let config_path = match args.config_path {
        Some(ref p) => &*p,
        None => crate::config::default_path(),
    };

    let ans = crate::config::read_config(config_path)
        .with_context(|| format!("Failed to read config from {:?}", config_path))?;

    log::info!("config_path = {}", config_path.display());
    log::info!("config:\n{:#?}", ans);

    Ok(ans)
}

fn build_app(config: crate::config::Config) -> App {
    let mut app = App::new();

    *app.root_mut() = crate::record_module::record_module(&config);

    app.provide(config)
        .provide_fn(crate::crypto::CryptoProvider)
        .provide_fn(crate::repo::RecordRepoProvider)
        .provide_fn(crate::cache::RecordCacheProvider);

    app
}

fn init_env_logger(log_filters: Option<&str>) {
    let mut builder = env_logger::Builder::from_default_env();
    if let Some(log) = log_filters {
        builder.parse_filters(log);
    }
    builder.init();
}

#[tokio::main]
async fn main() -> Result<(), BoxStdError> {
    let args = crate::args::get_args();

    init_env_logger(args.log.as_deref());

    let config = get_config(&args)?;

    let addr = &config.server.addr.clone();

    let app = build_app(config);

    let server =
        Server::listen(addr).with_context(|| format!("Failed to listen address: {}", &addr))?;

    log::info!("listening on http://{}", addr);
    server.run(app).await
}
