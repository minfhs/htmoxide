pub mod response;
pub mod component;
pub mod app;
pub mod state;
pub mod state_loader;
pub mod url_builder;
pub mod client_helpers;
pub mod state_urls_middleware;
pub mod body;

#[cfg(feature = "qs-forms")]
pub mod qs_form;

pub use htmoxide_macros::component;
pub use response::{Html, Page};
pub use app::{app, RouterExt, HtmxRouterExt};
pub use component::{Component, ComponentInfo};
pub use state::StateExtractor;
pub use state_loader::StateLoader;
pub use url_builder::{UrlBuilder, ComponentName};
pub use client_helpers::{cookie_cleaner_script, preserve_params, clear_input_handler};
pub use state_urls_middleware::{StateUrlsConfig, state_urls_middleware_impl};
pub use body::Body;

#[cfg(feature = "qs-forms")]
pub use qs_form::QsForm;

// Re-export inventory for macro use
#[doc(hidden)]
pub use inventory;

// Re-export common dependencies so users don't need to add them separately
pub use axum;
pub use maud;
pub use serde;
pub use serde_json;
pub use tokio;
pub use tower_cookies;

pub mod prelude {
    pub use crate::component;
    pub use crate::response::{Html, Page};
    pub use crate::app::{app, HtmxRouterExt};
    pub use crate::url_builder::UrlBuilder;
    pub use crate::state_loader::StateLoader;
    pub use crate::client_helpers::{cookie_cleaner_script, preserve_params, clear_input_handler};
    pub use crate::state_urls_middleware::StateUrlsConfig;
    pub use crate::body::Body;

    #[cfg(feature = "qs-forms")]
    pub use crate::qs_form::QsForm;

    // Re-export commonly used items from dependencies
    pub use maud::{html, Markup};
    pub use serde::{Deserialize, Serialize};
    pub use axum;
    pub use tower_cookies;
}
