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

pub fn build(config: &Config) -> Result<Router> {
    let svc = PastebinService::new(config)?;

    let middleware = tower::ServiceBuilder::new()
        .layer(HandleErrorLayer::new(handle_error))
        .buffer(4096)
        .load_shed()
        .rate_limit(config.security.max_qps.into(), Duration::from_secs(1))
        .into_inner();

    let router = Router::new()
        .route("/records/:key", get(find_record))
        .route("/records", put(save_record))
        .with_state(Arc::new(svc))
        .layer(middleware)
        .layer(DefaultBodyLimit::max(config.security.max_body_length));

    Ok(router)
}

async fn handle_error(err: BoxError) -> Json<PastebinError> {
    if err.is::<tower::load_shed::error::Overloaded>() {
        warn!("load shed: overloaded");
        return Json(PastebinErrorCode::Unavailable.into());
    }

    error!(?err);
    Json(PastebinErrorCode::InternalError.into())
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
///
/// -> JSON FindRecordOutput
pub async fn find_record(State(svc): AppState, Path(key): Path<String>) -> Response {
    json_result(svc.find_record(FindRecordInput { key }).await)
}

/// PUT /records    
///
/// JSON SaveRecordInput -> JSON SaveRecordOutput
pub async fn save_record(State(svc): AppState, Json(payload): Json<SaveRecordInput>) -> Response {
    json_result(svc.save_record(payload).await)
}
