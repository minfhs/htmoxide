use axum::{
    extract::{FromRequestParts, Query},
    http::request::Parts,
};
use serde::de::DeserializeOwned;

/// Extractor for component state from query parameters
#[derive(Debug, Clone)]
pub struct StateExtractor<T>(pub T);

impl<T, S> FromRequestParts<S> for StateExtractor<T>
where
    T: DeserializeOwned + Default,
    S: Send + Sync,
{
    type Rejection = std::convert::Infallible;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        // Try to extract from query params
        match Query::<T>::from_request_parts(parts, state).await {
            Ok(Query(value)) => Ok(StateExtractor(value)),
            Err(_) => Ok(StateExtractor(T::default())),
        }
    }
}

impl<T> std::ops::Deref for StateExtractor<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> std::ops::DerefMut for StateExtractor<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
