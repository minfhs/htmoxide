use axum::{
    Router,
    routing::get,
};
use tower_http::services::ServeDir;

/// Application builder
pub struct App<S = ()> {
    router: Router<S>,
}

impl App<()> {
    fn new() -> Self {
        Self {
            router: Router::new(),
        }
    }

    /// Add application state - transforms App<()> into App<S>
    pub fn with_state<S>(mut self, state: S) -> App<S>
    where
        S: Clone + Send + Sync + 'static,
    {
        // First, register all components without state
        for component in inventory::iter::<crate::ComponentInfo> {
            println!("Registering component: {} at {}", component.name, component.path);
            let handler = component.handler;
            self.router = self.router.route(
                component.path,
                get(move |req| handler(req))
            );
        }

        // Then add state
        App {
            router: self.router.with_state(state),
        }
    }
}

// Implementation for App without state
impl App<()> {
    /// Add a page route
    pub fn page<H, T>(mut self, path: &str, handler: H) -> Self
    where
        H: axum::handler::Handler<T, ()>,
        T: 'static,
    {
        self.router = self.router.route(path, get(handler));
        self
    }

    /// Add static file serving
    pub fn static_files(mut self, path: &str, dir: &str) -> Self {
        self.router = self.router.nest_service(path, ServeDir::new(dir));
        self
    }

    /// Build the final router with all registered components (no state)
    pub fn build(mut self) -> Router {
        // Register all components from the global registry
        for component in inventory::iter::<crate::ComponentInfo> {
            println!("Registering component: {} at {}", component.name, component.path);
            let handler = component.handler;
            self.router = self.router.route(
                component.path,
                get(move |req| handler(req))
            );
        }

        self.router
    }
}

// Implementation for App with state
impl<S> App<S>
where
    S: Clone + Send + Sync + 'static,
{
    /// Add a page route
    pub fn page<H, T>(mut self, path: &str, handler: H) -> Self
    where
        H: axum::handler::Handler<T, S>,
        T: 'static,
    {
        self.router = self.router.route(path, get(handler));
        self
    }

    /// Add static file serving
    pub fn static_files(mut self, path: &str, dir: &str) -> Self {
        self.router = self.router.nest_service(path, ServeDir::new(dir));
        self
    }

    /// Build the final router (components already registered in with_state)
    pub fn build(self) -> Router<S> {
        self.router
    }
}

/// Create a new application builder
pub fn app() -> App<()> {
    App::new()
}
