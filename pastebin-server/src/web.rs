use crate::config::Config;
use crate::dto::*;
use crate::error::PastebinError;
use crate::error::PastebinErrorCode;
use crate::svc::PastebinService;

use std::sync::Arc;
use std::time::Duration;

use axum::error_handling::HandleErrorLayer;
use axum::extract::DefaultBodyLimit;
use axum::extract::Path;
use axum::extract::State;
use axum::http::Request;
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::IntoResponse;
use axum::response::Response;
use axum::routing::get;
use axum::routing::put;
use axum::BoxError;
use axum::Json;
use axum::Router;

use anyhow::Result;
use serde::Serialize;
use tracing::error;
use tracing::warn;
use tracing_futures::Instrument;

pub fn build(config: &Config) -> Result<Router> {
    let svc = PastebinService::new(config)?;
    let state = Arc::new(svc);

    let tower_middleware = tower::ServiceBuilder::new()
        .layer(HandleErrorLayer::new(handle_error))
        .buffer(4096)
        .load_shed()
        .rate_limit(config.security.max_qps.into(), Duration::from_secs(1))
        .into_inner();

    let router = Router::new()
        .route("/api/records/:key", get(find_record))
        .route("/api/records", put(save_record))
        .with_state(state.clone())
        .layer(axum::middleware::from_fn_with_state(state, axum_middleware))
        .layer(tower_middleware)
        .layer(DefaultBodyLimit::max(config.security.max_http_body_length));

    Ok(router)
}

async fn handle_error(err: BoxError) -> Response {
    if err.is::<tower::load_shed::error::Overloaded>() {
        warn!("load shed: overloaded");
        return error_response(PastebinErrorCode::Unavailable.into());
    }

    error!(?err);
    error_response(PastebinErrorCode::InternalError.into())
}

async fn axum_middleware<B>(State(svc): AppState, req: Request<B>, next: Next<B>) -> Response {
    let _svc = svc;

    let x_forwarded_for = req.headers().get("x-forwarded-for");
    let x_real_ip = req.headers().get("x-real-ip");
    let span = tracing::debug_span!(
        "axum_middleware",
        method = %req.method(),
        path = %req.uri().path(),
        ?x_forwarded_for,
        ?x_real_ip
    );

    let res = next.run(req).instrument(span).await;

    // hide error details from serde_json
    if res.status() == StatusCode::UNPROCESSABLE_ENTITY {
        return StatusCode::UNPROCESSABLE_ENTITY.into_response();
    }

    res
}

fn json_result<T>(ret: Result<T, PastebinError>) -> Response
where
    T: Serialize,
{
    match ret {
        Ok(val) => Json(val).into_response(),
        Err(err) => error_response(err),
    }
}

fn error_response(err: PastebinError) -> Response {
    let status = err.code.status();
    let mut res = Json(err).into_response();
    *res.status_mut() = status;
    res
}

type AppState = State<Arc<PastebinService>>;

/// GET /api/records/:key
///
/// -> JSON FindRecordOutput
#[tracing::instrument(skip(svc))]
pub async fn find_record(State(svc): AppState, Path(key): Path<String>) -> Response {
    json_result(svc.find_record(FindRecordInput { key }).await)
}

/// PUT /api/records    
///
/// JSON SaveRecordInput -> JSON SaveRecordOutput
#[tracing::instrument(skip(svc, payload))]
pub async fn save_record(State(svc): AppState, Json(payload): Json<SaveRecordInput>) -> Response {
    json_result(svc.save_record(payload).await)
}
