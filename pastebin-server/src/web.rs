use crate::config::Config;
use crate::dto::*;
use crate::error::PastebinError;
use crate::error::PastebinErrorCode;
use crate::svc::PastebinService;

use std::sync::Arc;
use std::time::Duration;

use axum::error_handling::HandleErrorLayer;
use axum::extract::Path;
use axum::extract::State;
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

pub fn build(config: &Config) -> Result<Router> {
    let svc = PastebinService::new(config)?;

    let middleware = tower::ServiceBuilder::new()
        .layer(HandleErrorLayer::new(|err: BoxError| async move {
            error!(?err);
            Json(PastebinError::from(PastebinErrorCode::InternalError))
        }))
        .buffer(4096)
        .rate_limit(config.security.max_qps.into(), Duration::from_secs(1))
        .into_inner();

    Ok(Router::new()
        .route("/records/:key", get(find_record))
        .route("/records", put(save_record))
        .with_state(Arc::new(svc))
        .layer(middleware))
}

fn json_result<T, E>(ret: Result<T, E>) -> Response
where
    T: Serialize,
    E: Serialize,
{
    match ret {
        Ok(val) => Json(val).into_response(),
        Err(err) => Json(err).into_response(),
    }
}

type AppState = State<Arc<PastebinService>>;

/// GET /records/:key
async fn find_record(State(svc): AppState, Path(key): Path<String>) -> Response {
    json_result(svc.find_record(FindRecordInput { key }).await)
}

/// PUT /records
async fn save_record(State(svc): AppState, Json(payload): Json<SaveRecordInput>) -> Response {
    json_result(svc.save_record(payload).await)
}
