use htmoxide::prelude::*;
use axum::Extension;
use axum::extract::Query;
use std::sync::Arc;

use crate::layout::{head, header, navbar, common_styles};
use crate::components::*;
use crate::state::AppState;

pub async fn index() -> Page {
    html! {
        head {
            title { "htmoxide Demo" }
            script src="https://unpkg.com/htmx.org@1.9.10" {}
            (common_styles())
        }
        body {
            (header())
            (navbar(""))

            div.components {
                p { "Welcome to htmoxide! Choose a demo from the navigation above." }
                ul {
                    li {
                        a href="/simple" { "Simple Demo" }
                        " - Counter and greeter components with URL state"
                    }
                    li {
                        a href="/users" { "User Table" }
                        " - Sortable and filterable user table"
                    }
                }
            }
        }
    }
    .into()
}

pub async fn simple_page(Query(params): Query<std::collections::HashMap<String, String>>) -> Page {
    // Build query string from params
    let query_string = serde_urlencoded::to_string(&params).unwrap_or_default();

    // Extract state for each component
    let counter_state: CounterState = serde_urlencoded::from_str(&query_string).unwrap_or_default();
    let greeter_state: GreeterState = serde_urlencoded::from_str(&query_string).unwrap_or_default();

    // Create URL builders for components - they'll auto-detect page path from htmx headers
    let counter_url = UrlBuilder::new("/counter", &query_string).with_main_page("/simple");
    let greeter_url = UrlBuilder::new("/greeter", &query_string).with_main_page("/simple");

    html! {
        (head("Simple Demo - htmoxide"))
        body {
            (header())
            (navbar("simple"))

            div.components {
                (counter(counter_state, counter_url).await)
                (greeter(greeter_state, greeter_url).await)
            }
        }
    }
    .into()
}

pub async fn users_page(
    req: axum::http::Request<axum::body::Body>,
) -> Page {
    // Extract app state from extensions
    let app_state = req.extensions()
        .get::<Arc<AppState>>()
        .expect("AppState not found in extensions")
        .clone();
    
    let query_string = req.uri().query().unwrap_or("");
    let user_table_state: UserTableState = serde_urlencoded::from_str(query_string).unwrap_or_default();
    let user_table_url = UrlBuilder::new("/user_table", query_string).with_main_page("/users");

    html! {
        (head("User Table - htmoxide"))
        body {
            (header())
            (navbar("users"))

            div.components {
                // Render the user table component directly with shared state
                (user_table(user_table_state, user_table_url, Extension(app_state)).await)
            }
        }
    }
    .into()
}
