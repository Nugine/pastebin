mod cache;
mod config;
mod crypto;
mod dto;
mod endpoint;
mod error;
mod limiter;
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

use self::cache::RecordCache;
use self::config::Config;
use self::crypto::Crypto;
use self::limiter::TokenBucket;
use self::repo::RecordRepo;

use anyhow::Context;
use nuclear::core::{App, AppBuilder, DynEndpoint, InjectorExt, Result};
use nuclear::web::{self, router::SimpleRouter};
use std::path::PathBuf;
use std::time::Duration;
use structopt::StructOpt;

#[derive(StructOpt)]
pub struct Args {
    #[structopt(short = "l", long, help = "Log directives")]
    pub log: Option<String>,
    #[structopt(help = "Toml file path")]
    pub config_path: Option<PathBuf>,
}

fn get_config(args: &Args) -> Result<Config> {
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

fn init_env_logger(log_filters: Option<&str>) {
    let mut builder = env_logger::Builder::from_default_env();
    if let Some(log) = log_filters {
        builder.parse_filters(log);
    }
    builder.init();
}

async fn build_app(builder: AppBuilder) -> Result<App> {
    let mut router = SimpleRouter::new();

    let mut find = DynEndpoint::builder();
    let mut save = DynEndpoint::builder();

    let config = builder.try_inject_ref::<Config>()?;

    if let Some(ref limiter) = config.limiter {
        let one_second = Duration::from_secs(1);

        let find_qps = limiter.find_qps as u64;
        let mut find_limiter = TokenBucket::new(one_second, find_qps, find_qps);
        find_limiter.spawn_daemon();

        let save_qps = limiter.save_qps as u64;
        let mut save_limiter = TokenBucket::new(one_second, save_qps, save_qps);
        save_limiter.spawn_daemon();

        find = find.middleware(find_limiter);
        save = save.middleware(save_limiter);
    }

    let find = find.endpoint(crate::endpoint::find_record);
    let save = save.endpoint(crate::endpoint::save_record);

    router.at("/records/:key").get(find);
    router.at("/records").post(save);

    let catch_error = web::error::catch_error(|e: crate::error::RecordError| e.try_response());

    let app = builder.middleware(catch_error).endpoint(router);

    Ok(app)
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::from_args();
    init_env_logger(args.log.as_deref());

    let config = get_config(&args)?;

    let addr = &config.server.addr.clone();

    let app = App::resolver()
        .provide(config)
        .provide_type::<Crypto>()
        .provide_type::<RecordRepo>()
        .provide_type::<RecordCache>()
        .try_build_app(build_app)
        .await?;

    app.run(addr).await?;

    Ok(())
}
