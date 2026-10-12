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

/// Merge cookie values into component state and write the result back to cookies.
///
/// Query parameters win over cookies: a field is only taken from a cookie while it
/// still has its default value. Called by `#[component]`; a no-op unless htmoxide is
/// built with the `persist-state` feature. The feature check has to live here, since
/// a `cfg` in macro output is evaluated against the calling crate's features.
#[doc(hidden)]
#[cfg(feature = "persist-state")]
pub async fn hydrate_from_cookies<T>(parts: &mut Parts, state: T) -> T
where
    T: serde::Serialize + DeserializeOwned + Default,
{
    use serde_json::Value;
    use tower_cookies::{Cookie, Cookies};

    let Ok(cookies) = Cookies::from_request_parts(parts, &()).await else {
        return state;
    };

    let mut state = state;
    if let (Ok(Value::Object(defaults)), Ok(Value::Object(mut current))) =
        (serde_json::to_value(T::default()), serde_json::to_value(&state))
    {
        for (key, default_value) in &defaults {
            if current.get(key) != Some(default_value) {
                continue;
            }
            if let Some(value) = cookies.get(key).and_then(|c| parse_cookie_value(c.value())) {
                current.insert(key.clone(), value);
            }
        }
        if let Ok(merged) = serde_json::from_value(Value::Object(current)) {
            state = merged;
        }
    }

    // Save the resulting state so the next request without query params gets it back.
    if let Ok(Value::Object(fields)) = serde_json::to_value(&state) {
        for (key, value) in fields {
            let text = match value {
                Value::String(s) => s,
                Value::Number(n) => n.to_string(),
                Value::Bool(b) => b.to_string(),
                _ => continue,
            };
            if text.is_empty() {
                cookies.remove(Cookie::from(key));
            } else {
                let mut cookie = Cookie::new(key, text);
                cookie.set_path("/");
                cookies.add(cookie);
            }
        }
    }

    state
}

#[doc(hidden)]
#[cfg(not(feature = "persist-state"))]
pub async fn hydrate_from_cookies<T>(_parts: &mut Parts, state: T) -> T {
    state
}

#[cfg(feature = "persist-state")]
fn parse_cookie_value(value: &str) -> Option<serde_json::Value> {
    use serde_json::{Number, Value};

    if let Ok(n) = value.parse::<i64>() {
        Some(Value::Number(n.into()))
    } else if let Ok(n) = value.parse::<f64>() {
        Number::from_f64(n).map(Value::Number)
    } else if let Ok(b) = value.parse::<bool>() {
        Some(Value::Bool(b))
    } else if !value.is_empty() {
        Some(Value::String(value.to_string()))
    } else {
        None
    }
}
