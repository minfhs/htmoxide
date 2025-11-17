use axum::{
    extract::Request,
    middleware::Next,
    response::{IntoResponse, Response, Redirect},
};
use tower_cookies::Cookies;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

/// Configuration for state URLs middleware
#[derive(Clone, Debug)]
pub struct StateUrlsConfig {
    /// Cookie names to exclude from being added to query params
    /// Common examples: "token", "session_id", "csrf_token", "auth"
    pub denylist: Arc<HashSet<String>>,
}

impl StateUrlsConfig {
    /// Create a new config with an empty denylist
    pub fn new() -> Self {
        Self {
            denylist: Arc::new(HashSet::new()),
        }
    }

    /// Create a config with a custom denylist
    pub fn with_denylist<I, S>(items: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        Self {
            denylist: Arc::new(items.into_iter().map(|s| s.into()).collect()),
        }
    }

    /// Add items to the denylist
    pub fn deny<I, S>(mut self, items: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let mut denylist = (*self.denylist).clone();
        for item in items {
            denylist.insert(item.into());
        }
        self.denylist = Arc::new(denylist);
        self
    }
}

impl Default for StateUrlsConfig {
    fn default() -> Self {
        // Sensible defaults for common sensitive cookies
        Self::with_denylist([
            "token",
            "session",
            "session_id",
            "sessionid",
            "csrf",
            "csrf_token",
            "auth",
            "auth_token",
            "jwt",
            "bearer",
            "id",
        ])
    }
}

/// Middleware that redirects requests without query parameters to include cookie values
///
/// This middleware:
/// 1. Checks if the request has any query parameters
/// 2. If not, loads values from cookies (excluding denylisted ones) and redirects
/// 3. If yes, allows the request to proceed normally
///
/// This makes state from cookies immediately visible in the URL, enabling:
/// - Bookmarkable URLs that preserve state
/// - Shareable links with current state
/// - Browser back/forward button working correctly
///
/// # Security
/// Sensitive cookies (tokens, session IDs, etc.) are excluded via the denylist
pub async fn state_urls_middleware_impl(
    config: Arc<StateUrlsConfig>,
    cookies: Cookies,
    request: Request,
    next: Next,
) -> Response {
    let uri = request.uri();
    let path = uri.path();

    // Skip if this is an htmx request (already has state in URL or is a component update)
    if request.headers().get("HX-Request").is_some() {
        return next.run(request).await;
    }

    // Skip if query parameters already exist
    if uri.query().is_some() {
        return next.run(request).await;
    }

    // Collect cookies into query parameters, excluding denylisted ones
    let mut query_params: HashMap<String, String> = HashMap::new();

    for cookie in cookies.list() {
        let name = cookie.name();
        let value = cookie.value();

        // Skip denylisted cookies
        if config.denylist.contains(name) {
            continue;
        }

        // Skip empty values
        if !value.is_empty() {
            query_params.insert(name.to_string(), value.to_string());
        }
    }

    // If we have cookies, redirect to the same path with query params
    if !query_params.is_empty() {
        let query_string = serde_urlencoded::to_string(&query_params)
            .unwrap_or_default();
        let redirect_url = format!("{}?{}", path, query_string);
        return Redirect::to(&redirect_url).into_response();
    }

    // No cookies found, proceed normally
    next.run(request).await
}
