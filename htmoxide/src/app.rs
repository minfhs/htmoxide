use axum::{
    Extension, Router,
    routing::{delete, get, patch, post, put},
};
use std::sync::Arc;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;

/// Create a new application with auto-registered components
///
/// Returns a `Router<()>` that components are registered on.
/// Use `.app_state()` to inject application state that components can access.
///
/// # Example with state
/// ```ignore
/// #[derive(Clone)]
/// struct AppState {
///     counter: Arc<Mutex<i32>>,
/// }
///
/// let state = Arc::new(AppState {
///     counter: Arc::new(Mutex::new(0)),
/// });
///
/// let app = app()
///     .app_state(state)
///     .page("/", index);
/// ```
///
/// # Example without state
/// ```ignore
/// let app = app()
///     .page("/", index);
/// ```
pub fn app() -> Router {
    let mut router = Router::new();

    // Register all components from the global registry
    for component in inventory::iter::<crate::ComponentInfo> {
        println!(
            "Registering component: {} at {} ({})",
            component.name, component.path, component.method
        );
        let handler = component.handler;

        // Route based on HTTP method
        #[allow(clippy::redundant_closure)]
        let method_service = match component.method {
            "POST" => post(move |req| handler(req)),
            "PUT" => put(move |req| handler(req)),
            "DELETE" => delete(move |req| handler(req)),
            "PATCH" => patch(move |req| handler(req)),
            _ => get(move |req| handler(req)), // Default to GET
        };

        router = router.route(component.path, method_service);
    }

    router
}

/// Helper trait to add features to Router
pub trait RouterExt<S>: Sized {
    /// Add a page route
    fn page<H, T>(self, path: &str, handler: H) -> Self
    where
        H: axum::handler::Handler<T, S>,
        T: 'static;

    /// Add static file serving
    fn static_files(self, path: &str, dir: &str) -> Self;

    /// Add application state that components can access via Extension<Arc<AppState>>
    fn app_state<AppState>(self, state: Arc<AppState>) -> Self
    where
        AppState: Clone + Send + Sync + 'static;
}

impl<S> RouterExt<S> for Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    fn page<H, T>(self, path: &str, handler: H) -> Self
    where
        H: axum::handler::Handler<T, S>,
        T: 'static,
    {
        self.route(path, get(handler))
    }

    fn static_files(self, path: &str, dir: &str) -> Self {
        self.nest_service(path, ServeDir::new(dir))
    }

    fn app_state<AppState>(self, state: Arc<AppState>) -> Self
    where
        AppState: Clone + Send + Sync + 'static,
    {
        self.layer(Extension(state))
    }
}

/// HTMX-specific router extensions
pub trait HtmxRouterExt<S>: Sized {
    /// Adds all required HTMX system layers.
    ///
    /// This includes:
    /// - `CookieManagerLayer` for cookie management (required for empty form value handling)
    ///
    /// # Important: Call this AFTER adding all routes
    ///
    /// Axum middleware only applies to routes that exist before the layer is added.
    /// Always add all your routes first, then call `.htmx()`.
    ///
    /// # Example
    /// ```ignore
    /// let app = app()
    ///     .route("/login", get(login_page))
    ///     .route("/dashboard", get(dashboard))
    ///     .htmx()  // Add HTMX layers AFTER all routes
    ///     .layer(Extension(app_state));  // Optional app-specific layers
    /// ```
    fn htmx(self) -> Self;

    /// Enables automatic state URLs - redirects page loads to include cookie values in URL.
    ///
    /// When enabled, requests to pages without query parameters will be redirected
    /// to include cookie values as query parameters. This makes state visible in the URL
    /// and enables bookmarking/sharing with current state.
    ///
    /// By default, sensitive cookies are excluded (token, session, auth, etc.).
    /// Use `with_state_urls_custom()` to customize the denylist.
    ///
    /// # Example
    /// User visits `/simple` with `count=3` in cookies
    /// → Redirected to `/simple?count=3`
    ///
    /// # Note
    /// - Only affects non-htmx requests (initial page loads)
    /// - Skips requests that already have query parameters
    /// - Must be called AFTER `.htmx()` to ensure cookies are available
    ///
    /// ```ignore
    /// let app = app()
    ///     .route("/", index_page)
    ///     .htmx()
    ///     .with_state_urls();  // Enable with default denylist
    /// ```
    fn with_state_urls(self) -> Self;

    /// Enables state URLs with a custom configuration.
    ///
    /// # Example with custom denylist
    /// ```ignore
    /// use htmoxide::StateUrlsConfig;
    ///
    /// let config = StateUrlsConfig::new()
    ///     .deny(["my_secret", "internal_state"]);
    ///
    /// let app = app()
    ///     .route("/", index_page)
    ///     .htmx()
    ///     .with_state_urls_custom(config);
    /// ```
    fn with_state_urls_custom(self, config: crate::StateUrlsConfig) -> Self;
}

impl<S> HtmxRouterExt<S> for Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    fn htmx(self) -> Self {
        self.layer(CookieManagerLayer::new())
    }

    fn with_state_urls(self) -> Self {
        self.with_state_urls_custom(crate::StateUrlsConfig::default())
    }

    fn with_state_urls_custom(self, config: crate::StateUrlsConfig) -> Self {
        let config = Arc::new(config);
        self.layer(axum::middleware::from_fn(move |cookies, request, next| {
            let config = config.clone();
            crate::state_urls_middleware::state_urls_middleware_impl(config, cookies, request, next)
        }))
    }
}
