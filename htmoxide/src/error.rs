// maybe consider this: https://docs.rs/thiserror/latest/thiserror/

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use std::error::Error as StdError;
use std::fmt;

#[derive(Debug)]
pub enum HtmoxideError {
    InvalidQueryString(String),
    Extraction { status: StatusCode },
}

impl fmt::Display for HtmoxideError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidQueryString(message) => {
                write!(f, "Invalid query string: {message}")
            }
            Self::Extraction { status } => {
                write!(f, "Extractor failed with status {status}")
            }
        }
    }
}

impl HtmoxideError {
    pub fn from_rejection<R>(rejection: R) -> Self
    where
        R: IntoResponse,
    {
        let response = rejection.into_response();

        Self::Extraction {
            status: response.status(),
        }
    }
}

impl IntoResponse for HtmoxideError {
    fn into_response(self) -> Response {
        match self {
            HtmoxideError::InvalidQueryString(message) => {
                (StatusCode::BAD_REQUEST, message).into_response()
            }

            HtmoxideError::Extraction { status } => {
                (status, "Request extraction failed").into_response()
            }
        }
    }
}

impl StdError for HtmoxideError {}
