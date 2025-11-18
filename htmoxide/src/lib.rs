pub mod app;
pub mod body;
pub mod client_helpers;
pub mod component;
pub mod response;
pub mod state;
pub mod state_loader;
pub mod state_urls_middleware;
pub mod url_builder;

#[cfg(feature = "qs-forms")]
pub mod qs_form;

pub use app::{HtmxRouterExt, RouterExt, app};
pub use body::Body;
pub use client_helpers::{clear_input_handler, cookie_cleaner_script, preserve_params};
pub use component::{Component, ComponentInfo};
pub use htmoxide_macros::component;
pub use response::{Html, Page};
pub use state::StateExtractor;
pub use state_loader::StateLoader;
pub use state_urls_middleware::{StateUrlsConfig, state_urls_middleware_impl};
pub use url_builder::{ComponentName, UrlBuilder};

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
    pub use crate::app::{HtmxRouterExt, app};
    pub use crate::body::Body;
    pub use crate::client_helpers::{clear_input_handler, cookie_cleaner_script, preserve_params};
    pub use crate::component;
    pub use crate::response::{Html, Page};
    pub use crate::state_loader::StateLoader;
    pub use crate::state_urls_middleware::StateUrlsConfig;
    pub use crate::url_builder::UrlBuilder;

    #[cfg(feature = "qs-forms")]
    pub use crate::qs_form::QsForm;

    // Re-export commonly used items from dependencies
    pub use axum;
    pub use maud::{Markup, html};
    pub use serde::{Deserialize, Serialize};
    pub use tower_cookies;
}
