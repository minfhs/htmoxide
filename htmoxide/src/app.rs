use axum::{
    Router,
    routing::get,
};
use tower_http::services::ServeDir;

/// Application builder
pub struct App {
    router: Router,
}

impl App {
    fn new() -> Self {
        Self {
            router: Router::new(),
        }
    }

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

    /// Build the final router with all registered components
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

/// Create a new application builder
pub fn app() -> App {
    App::new()
}
