use nuclear::http::StatusCode;
use nuclear::web::reply::Json;
use nuclear::Response;

use serde::{Deserialize, Serialize};

#[repr(u16)]
#[derive(Debug, Clone, Copy, thiserror::Error)]
pub enum PastebinError {
    #[error("Can not parse key")]
    BadKey = 1001,

    #[error("Can not find record")]
    NotFound = 1002,

    #[error("Too long expiration")]
    TooLongExpirations = 1003,

    #[error("Too long content")]
    TooLongContent = 1004,

    #[error("Redis error")]
    RedisError = 1005,

    #[error("Json error")]
    JsonError = 1006,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiErrorRes {
    pub code: u16,
    pub message: String,
}

impl PastebinError {
    pub fn res(self) -> nuclear::Result<Response> {
        use PastebinError::*;

        let status: StatusCode = match self {
            BadKey => StatusCode::BAD_REQUEST,
            TooLongExpirations => StatusCode::BAD_REQUEST,
            TooLongContent => StatusCode::BAD_REQUEST,
            NotFound => StatusCode::NOT_FOUND,
            RedisError | JsonError => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let res = ApiErrorRes {
            code: self as _,
            message: self.to_string(),
        };

        Json::new(status, res).res()
    }
}
