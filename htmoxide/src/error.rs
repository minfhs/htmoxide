// maybe consider this: https://docs.rs/thiserror/latest/thiserror/

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use std::error::Error as StdError;
use std::fmt;

#[derive(Debug)]
pub enum HtmoxideError {
    InvalidQueryString(String),
    Extraction { status: StatusCode },
    UnresolvedPathParameters { path: String, params: Vec<String> },
    QuerySerialization(String),
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
            Self::UnresolvedPathParameters { path, params } => {
                write!(
                    f,
                    "Unresolved path parameters for path '{}': {:?}",
                    path, params
                )
            }
            Self::QuerySerialization(message) => {
                write!(f, "Failed to serialize query parameters: {message}")
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

// Implement IntoResponse for HtmoxideError so handlers that return a response
// directly can still propagate htmoxide errors without requiring a Result<T, E> return type.
impl IntoResponse for HtmoxideError {
    fn into_response(self) -> Response {
        match self {
            HtmoxideError::InvalidQueryString(message) => {
                (StatusCode::BAD_REQUEST, message).into_response()
            }

            HtmoxideError::Extraction { status } => {
                (status, "Request extraction failed").into_response()
            }
            HtmoxideError::UnresolvedPathParameters { path, params } => (
                StatusCode::BAD_REQUEST,
                format!(
                    "Unresolved path parameters for path '{}': {:?}",
                    path, params
                ),
            )
                .into_response(),
            HtmoxideError::QuerySerialization(message) => {
                (StatusCode::INTERNAL_SERVER_ERROR, message).into_response()
            }
        }
    }
}

impl StdError for HtmoxideError {}
