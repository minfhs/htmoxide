pub mod response;
pub mod component;
pub mod app;
pub mod state;
pub mod state_loader;
pub mod url_builder;

pub use htmoxide_macros::component;
pub use response::{Html, Page};
pub use app::{app, RouterExt};
pub use component::{Component, ComponentInfo};
pub use state::StateExtractor;
pub use state_loader::StateLoader;
pub use url_builder::UrlBuilder;

// Re-export inventory for macro use
#[doc(hidden)]
pub use inventory;

// Re-export for macro use
#[doc(hidden)]
pub use serde_json;
#[doc(hidden)]
pub use tower_cookies;

pub mod prelude {
    pub use crate::component;
    pub use crate::response::{Html, Page};
    pub use crate::app::app;
    pub use crate::url_builder::UrlBuilder;
    pub use crate::state_loader::StateLoader;
    pub use maud::{html, Markup};
    pub use serde::{Deserialize, Serialize};
}
