#![feature(is_terminal)]
#![deny(clippy::all)]

use pastebin_server::config::Config;
use pastebin_server::svc::PastebinService;

use std::io::IsTerminal;
use std::net::TcpListener;

use anyhow::Context;
use anyhow::Result;
use camino::Utf8PathBuf;
use clap::Parser;
use tracing::info;

#[derive(clap::Parser)]
struct Opt {
    #[clap(long)]
    #[clap(default_value = "pastebin-server.toml")]
    pub config: Utf8PathBuf,
}

#[tokio::main]
async fn main() -> Result<()> {
    setup_tracing();

    let opt = Opt::parse();

    let config = Config::from_toml(&opt.config)
        .with_context(|| format!("Failed to read config from {:?}", opt.config))?;

    let addr = config.server.bind_addr.clone();

    let svc = PastebinService::new(config)?;
    let app = pastebin_server::web::build(svc);

    let listener = TcpListener::bind(&addr)?;
    let server = axum::Server::from_tcp(listener)?
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal());

    info!("listening on {}", addr);

    server.await?;

    Ok(())
}

fn setup_tracing() {
    use tracing_subscriber::filter::{EnvFilter, LevelFilter};

    let enable_color = std::io::stdout().is_terminal();

    let env_filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .from_env_lossy();

    tracing_subscriber::fmt()
        .pretty()
        .with_env_filter(env_filter)
        .with_ansi(enable_color)
        .init()
}

async fn shutdown_signal() {
    let _ = tokio::signal::ctrl_c().await;
}
