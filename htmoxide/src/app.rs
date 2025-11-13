use axum::{
    Router,
    routing::get,
    Extension,
};
use tower_http::services::ServeDir;
use std::sync::Arc;

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
        println!("Registering component: {} at {}", component.name, component.path);
        let handler = component.handler;
        router = router.route(
            component.path,
            get(move |req| handler(req))
        );
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
