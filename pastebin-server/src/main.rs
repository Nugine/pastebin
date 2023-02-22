#![feature(is_terminal)]
#![deny(clippy::all)]

use camino::Utf8Path;
use pastebin_server::config::Config;

use std::io::IsTerminal;
use std::net::TcpListener;

use anyhow::Context;
use anyhow::Result;
use axum::Router;
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

    let config = load_config(&opt.config)?;
    let app = pastebin_server::web::build(&config)?;
    serve(app, &config.server.bind_addr).await?;

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

fn load_config(path: &Utf8Path) -> Result<Config> {
    Config::from_toml(path).with_context(|| format!("Failed to read config from {:?}", path))
}

async fn serve(app: Router, addr: &str) -> Result<()> {
    let listener = TcpListener::bind(addr)?;
    let server = axum::Server::from_tcp(listener)?
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal());

    info!("listening on {}", addr);
    server.await?;
    Ok(())
}

async fn shutdown_signal() {
    let _ = tokio::signal::ctrl_c().await;
}
