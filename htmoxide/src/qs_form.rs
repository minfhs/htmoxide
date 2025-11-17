//! Query string form extractor with support for array fields.
//!
//! This module provides `QsForm<T>`, a form extractor that uses `serde_qs`
//! instead of `serde_urlencoded`. This enables proper handling of PHP-style
//! array notation in form fields (e.g., `field[]=value1&field[]=value2`).
//!
//! # Example
//!
//! ```rust,ignore
//! use htmoxide::prelude::*;
//! use serde::{Deserialize, Serialize};
//!
//! #[derive(Deserialize, Serialize, Default, Clone)]
//! struct MyForm {
//!     title: String,
//!     tags: Vec<String>,  // Will parse from tags[]=foo&tags[]=bar
//! }
//!
//! #[component(method = "POST")]
//! async fn my_handler(
//!     state: MyState,
//!     url: UrlBuilder,
//!     Body(QsForm(form)): Body<QsForm<MyForm>>,
//! ) -> Html {
//!     // form.tags contains ["foo", "bar"]
//!     html! { div { "Received " (form.tags.len()) " tags" } }.into()
//! }
//! ```

use axum::body::Bytes;
use axum::extract::{FromRequest, Request};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::de::DeserializeOwned;

/// Form extractor that uses `serde_qs` for parsing.
///
/// This extractor supports:
/// - Array fields with bracket notation: `field[]=value1&field[]=value2`
/// - Nested structures: `user[name]=John&user[age]=30`
/// - Indexed arrays: `items[0]=a&items[1]=b`
/// - Complex nested structures via `serde_qs`
///
/// # Usage with `Body<QsForm<T>>`
///
/// When using with htmoxide's `Body` wrapper (for components), use this pattern:
///
/// ```rust,ignore
/// Body(QsForm(form)): Body<QsForm<MyForm>>
/// ```
///
/// # Comparison with `axum::Form`
///
/// - `axum::Form` uses `serde_urlencoded` which doesn't support array notation
/// - `QsForm` uses `serde_qs` which handles arrays, nested objects, and more complex structures
///
/// # Feature Flag
///
/// Requires the `qs-forms` feature to be enabled in `htmoxide`.
#[derive(Debug, Clone, Copy, Default)]
pub struct QsForm<T>(pub T);

impl<T, S> FromRequest<S> for QsForm<T>
where
    T: DeserializeOwned,
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let bytes = Bytes::from_request(req, state).await.map_err(|err| {
            (
                StatusCode::BAD_REQUEST,
                format!("Failed to read request body: {}", err),
            )
                .into_response()
        })?;

        let body_str = String::from_utf8_lossy(&bytes);
        let decoded = urlencoding::decode(&body_str).map_err(|err| {
            (
                StatusCode::BAD_REQUEST,
                format!("Failed to URL decode form data: {}", err),
            )
                .into_response()
        })?;

        let value = serde_qs::from_str(&decoded).map_err(|err| {
            (
                StatusCode::BAD_REQUEST,
                format!("Failed to deserialize form data: {}", err),
            )
                .into_response()
        })?;

        Ok(QsForm(value))
    }
}

impl<T> std::ops::Deref for QsForm<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> std::ops::DerefMut for QsForm<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
