pub mod response;
pub mod component;
pub mod app;
pub mod state;
pub mod url_builder;

pub use htmoxide_macros::component;
pub use response::{Html, Page};
pub use app::{app, RouterExt};
pub use component::{Component, ComponentInfo};
pub use state::StateExtractor;
pub use url_builder::UrlBuilder;

// Re-export inventory for macro use
#[doc(hidden)]
pub use inventory;

pub mod prelude {
    pub use crate::component;
    pub use crate::response::{Html, Page};
    pub use crate::app::app;
    pub use crate::url_builder::UrlBuilder;
    pub use maud::{html, Markup};
    pub use serde::{Deserialize, Serialize};
}
