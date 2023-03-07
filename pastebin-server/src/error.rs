use axum::http::StatusCode;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Debug, Serialize, Deserialize)]
pub struct PastebinError {
    pub code: PastebinErrorCode,
    pub message: String,
}

#[repr(u16)]
#[derive(Debug, Clone, Copy, Serialize_repr, Deserialize_repr)]
pub enum PastebinErrorCode {
    InternalError = 1001,
    Unavailable = 1002,

    BadKey = 2001,
    TooLongExpirations = 2002,
    TooLongContent = 2003,

    NotFound = 3001,
}

impl PastebinErrorCode {
    pub fn status(&self) -> StatusCode {
        use PastebinErrorCode::*;

        match self {
            InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            Unavailable => StatusCode::SERVICE_UNAVAILABLE,
            BadKey => StatusCode::BAD_REQUEST,
            TooLongExpirations => StatusCode::BAD_REQUEST,
            TooLongContent => StatusCode::BAD_REQUEST,
            NotFound => StatusCode::NOT_FOUND,
        }
    }
}

impl From<PastebinErrorCode> for PastebinError {
    fn from(code: PastebinErrorCode) -> Self {
        let message = format!("{code:?}");
        Self { code, message }
    }
}
