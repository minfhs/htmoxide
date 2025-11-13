use axum::{
    response::Response,
    http::Request,
    body::Body,
};
use std::future::Future;
use std::pin::Pin;

/// Type alias for component handler functions
pub type ComponentHandler = fn(Request<Body>) -> Pin<Box<dyn Future<Output = Response> + Send>>;

/// Information about a registered component
#[derive(Clone)]
pub struct ComponentInfo {
    pub name: &'static str,
    pub path: &'static str,
    pub handler: ComponentHandler,
}

impl ComponentInfo {
    pub const fn new(name: &'static str, path: &'static str, handler: ComponentHandler) -> Self {
        Self { name, path, handler }
    }
}

/// Trait for component registration
pub trait Component {
    fn info() -> ComponentInfo;
}

// Global component registry using inventory
inventory::collect!(ComponentInfo);
