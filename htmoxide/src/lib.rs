pub mod response;
pub mod component;
pub mod app;
pub mod state;

pub use htmoxide_macros::{component, component_url};
pub use response::{Html, Page};
pub use app::{App, app};
pub use component::{Component, ComponentInfo};
pub use state::StateExtractor;

// Re-export inventory for macro use
#[doc(hidden)]
pub use inventory;

pub mod prelude {
    pub use crate::{component, component_url};
    pub use crate::response::{Html, Page};
    pub use crate::app::app;
    pub use maud::{html, Markup};
    pub use serde::{Deserialize, Serialize};
}
