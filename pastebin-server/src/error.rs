use nuclear::{
    core::{Responder, Response, Result},
    http::StatusCode,
    web,
};
use serde::{Deserialize, Serialize};

#[repr(u16)]
#[derive(Debug, Clone, Copy, thiserror::Error)]
pub enum RecordError {
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
pub struct ErrorRes {
    pub code: u16,
    pub message: String,
}

impl RecordError {
    pub fn try_response(self: Self) -> Result<Response> {
        use RecordError::*;

        let status: StatusCode = match self {
            BadKey => StatusCode::BAD_REQUEST,
            TooLongExpirations => StatusCode::BAD_REQUEST,
            TooLongContent => StatusCode::BAD_REQUEST,
            NotFound => StatusCode::NOT_FOUND,
            RedisError | JsonError => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let res: ErrorRes = ErrorRes {
            code: self as _,
            message: self.to_string(),
        };

        let mut res = web::reply::json(res).try_response()?;
        res.set_status(status);
        Ok(res)
    }
}
