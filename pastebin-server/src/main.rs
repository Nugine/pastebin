#![deny(clippy::all)]

use std::net::SocketAddr;

use axum::response::Html;
use axum::routing::get;
use axum::Router;

use anyhow::Result;
use tracing::error;
use tracing::info;

fn setup_tracing() {
    use tracing_subscriber::EnvFilter;

    tracing_subscriber::fmt()
        .pretty()
        .with_env_filter(EnvFilter::from_default_env())
        .init()
}

#[tokio::main]
async fn main() -> Result<()> {
    setup_tracing();

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let app = build_app();

    let server = axum::Server::bind(&addr).serve(app.into_make_service());
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

fn build_app() -> Router {
    Router::new().route("/", get(handler))
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}
