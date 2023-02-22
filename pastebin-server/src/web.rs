use crate::dto::*;
use crate::svc::PastebinService;

use std::sync::Arc;

use axum::extract::Path;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::response::Response;
use axum::routing::get;
use axum::routing::put;
use axum::Json;
use axum::Router;

use serde::Serialize;

pub fn build(svc: PastebinService) -> Router {
    Router::new()
        .route("/records/:key", get(find_record))
        .route("/records", put(save_record))
        .with_state(Arc::new(svc))
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
