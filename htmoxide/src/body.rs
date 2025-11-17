//! Body extractor wrapper for components
//!
//! This module provides a `Body<T>` wrapper that explicitly marks parameters
//! as body extractors (Form, Json, etc.) in component signatures.

use axum::extract::FromRequest;
use axum::response::IntoResponse;
use std::ops::{Deref, DerefMut};

/// Wrapper for body extractors in components
///
/// Use this to explicitly mark form data, JSON, or other body extractors
/// in your component signatures. The Body wrapper must be the last parameter.
///
/// # Example
/// ```ignore
/// use htmoxide::prelude::*;
/// use axum::extract::Form;
///
/// #[derive(Deserialize)]
/// struct CreateTodo {
///     title: String,
/// }
///
/// #[component(method = "POST")]
/// async fn create_todo(
///     state: TodoState,
///     url: UrlBuilder,
///     Body(form): Body<Form<CreateTodo>>,
/// ) -> Html {
///     // form is of type Form<CreateTodo>
///     let title = form.title;
///     // ...
/// }
/// ```
#[derive(Debug)]
pub struct Body<T>(pub T);

impl<T> Body<T> {
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T> Deref for Body<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Body<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

// Implement FromRequest so Body<T> can extract from request body
impl<T, S> FromRequest<S> for Body<T>
where
    T: FromRequest<S>,
    S: Send + Sync,
{
    type Rejection = T::Rejection;

    async fn from_request(
        req: axum::http::Request<axum::body::Body>,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        T::from_request(req, state).await.map(Body)
    }
}

impl<T> IntoResponse for Body<T>
where
    T: IntoResponse,
{
    fn into_response(self) -> axum::response::Response {
        self.0.into_response()
    }
}
