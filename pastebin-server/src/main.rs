#![feature(is_terminal)]
#![deny(clippy::all)]

use pastebin_server::config::Config;
use pastebin_server::dto::FindRecordInput;
use pastebin_server::dto::SaveRecordInput;
use pastebin_server::svc::PastebinService;

use std::io::IsTerminal;
use std::net::TcpListener;
use std::sync::Arc;

use axum::extract::Path;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::response::Response;
use axum::routing::get;
use axum::routing::put;
use axum::Json;
use axum::Router;

use anyhow::Context;
use anyhow::Result;
use camino::Utf8PathBuf;
use clap::Parser;
use tracing::error;
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
    let app = build_app(svc);

    let listener = TcpListener::bind(&addr)?;
    let server = axum::Server::from_tcp(listener)?.serve(app.into_make_service());
    info!("listening on {}", addr);

    let task = tokio::spawn(async move {
        if let Err(err) = server.await {
            error!(?err);
        }
    });

    tokio::signal::ctrl_c().await?;
    task.abort();

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

/// GET /records/:key
async fn find_record(State(svc): State<Arc<PastebinService>>, Path(key): Path<String>) -> Response {
    let input = FindRecordInput { key };
    match svc.find_record(input).await {
        Ok(output) => Json(output).into_response(),
        Err(error) => Json(error).into_response(),
    }
}

/// PUT /records
async fn save_record(
    State(svc): State<Arc<PastebinService>>,
    Json(payload): Json<SaveRecordInput>,
) -> Response {
    let input = payload;
    match svc.save_record(input).await {
        Ok(output) => Json(output).into_response(),
        Err(error) => Json(error).into_response(),
    }
}

fn build_app(svc: PastebinService) -> Router {
    Router::new()
        .route("/records/:key", get(find_record))
        .route("/records", put(save_record))
        .with_state(Arc::new(svc))
}
