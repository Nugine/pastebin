use pastebin_server::app::App;
use pastebin_server::config::Config;

use anyhow::{Context, Result};
use camino::Utf8PathBuf;
use nuclear::prelude::Handler;
use structopt::StructOpt;

fn setup_tracing() {
    use tracing_subscriber::util::SubscriberInitExt;
    use tracing_subscriber::{fmt, EnvFilter};

    tracing_subscriber::fmt()
        .event_format(fmt::format::Format::default().pretty())
        .with_env_filter(EnvFilter::from_default_env())
        .with_timer(fmt::time::ChronoLocal::rfc3339())
        .finish()
        .init();
}

#[derive(StructOpt)]
struct Opt {
    #[structopt(long, help = "Toml file path")]
    pub config: Option<Utf8PathBuf>,
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    setup_tracing();

    let opt = Opt::from_args();

    let config_path = match opt.config.as_deref() {
        Some(p) => p,
        None => Config::default_path(),
    };

    let config = Config::from_toml(config_path)
        .with_context(|| format!("Failed to read config from {:?}", config_path))?;

    tracing::info!("config_path = {}", config_path);
    tracing::info!("config:\n{:#?}", config);

    let addr = config.server.addr.clone();
    let app = App::new(config)?;
    app.into_handler().into_server().run(addr).await
}
