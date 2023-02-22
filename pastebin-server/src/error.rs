use axum::http::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PastebinError {
    pub code: u16,
    pub message: String,
}

#[repr(u16)]
#[derive(Debug, Clone, Copy)]
pub enum PastebinErrorCode {
    InternalError = 1001,

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
            BadKey => StatusCode::BAD_REQUEST,
            TooLongExpirations => StatusCode::BAD_REQUEST,
            TooLongContent => StatusCode::BAD_REQUEST,
            NotFound => StatusCode::NOT_FOUND,
        }
    }
}

impl From<PastebinErrorCode> for PastebinError {
    fn from(ec: PastebinErrorCode) -> Self {
        let code = ec as u16;
        let message = format!("{ec:?}");
        Self { code, message }
    }
}
