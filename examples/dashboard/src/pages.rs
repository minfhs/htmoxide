use htmoxide::prelude::*;
use axum::Extension;
use axum::extract::Query;
use axum::response::{IntoResponse, Redirect};
use std::sync::Arc;
use tower_cookies::Cookies;

use crate::layout::{head, header, navbar};
use crate::components::*;
use crate::state::AppState;
use crate::auth::AuthSession;

pub async fn index(auth_session: AuthSession) -> Page {
    let user = &auth_session.user;
    let username = user.as_ref().map(|u| u.name.as_str());
    
    html! {
        (head("htmoxide Demo"))
        body {
            (header(username))
            (navbar("home"))

            main.container {
                div.components {
                    p { "Welcome to htmoxide! Choose a demo from the navigation above." }
                    ul {
                        li {
                            a href="/simple" { "Simple Demo" }
                            " - Counter and greeter components with URL state"
                        }
                        li {
                            a href="/users" { "User Table" }
                            " - Sortable and filterable user table (requires login)"
                        }
                        li {
                            a href="/combined" { "Combined View" }
                            " - Two-column layout with all components (requires login)"
                        }
                    }
                }
            }
        }
    }
    .into()
}

pub async fn simple_page(
    auth_session: AuthSession,
    cookies: Cookies,
    query: Query<std::collections::HashMap<String, String>>
) -> Page {
    let user = &auth_session.user;
    let username = user.as_ref().map(|u| u.name.as_str());

    html! {
        (head("Simple Demo - htmoxide"))
        body {
            (header(username))
            (navbar("simple"))

            main.container {
                form id="page-state" {
                    div.components {
                        (counter(CounterState::default(), UrlBuilder::new("/counter", "").with_main_page("/simple"), cookies.clone(), query.clone()).await)
                        (greeter(GreeterState::default(), UrlBuilder::new("/greeter", "").with_main_page("/simple"), cookies, query).await)
                    }
                }
            }
        }
    }
    .into()
}

pub async fn users_page(
    auth_session: AuthSession,
    cookies: Cookies,
    query: Query<std::collections::HashMap<String, String>>,
    Extension(app_state): Extension<Arc<AppState>>,
) -> impl IntoResponse {
    // Require authentication for this page
    let user = match &auth_session.user {
        Some(user) => user,
        None => return Redirect::to("/login?redirect=/users").into_response(),
    };

    let username = Some(user.name.as_str());

    // Clone auth_session since we need it in two places
    let auth_for_component = auth_session.clone();

    let page: Page = html! {
        (head("User Table - htmoxide"))
        body {
            (header(username))
            (navbar("users"))

            main.container {
                div.components {
                    // Render the user table component directly with shared state
                    (user_table(UserTableState::default(), UrlBuilder::new("/user_table", "").with_main_page("/users"), Extension(app_state), cookies, query, auth_for_component).await)
                }
            }
        }
    }
    .into();

    page.into_response()
}

pub async fn combined_page(
    auth_session: AuthSession,
    cookies: Cookies,
    query: Query<std::collections::HashMap<String, String>>,
    Extension(app_state): Extension<Arc<AppState>>,
) -> impl IntoResponse {
    // Require authentication for this page
    let user = match &auth_session.user {
        Some(user) => user,
        None => return Redirect::to("/login?redirect=/combined").into_response(),
    };

    let username = Some(user.name.as_str());

    // Clone auth_session for the user_table component
    let auth_for_component = auth_session.clone();

    let page: Page = html! {
        (head("Combined View - htmoxide"))
        body {
            (header(username))
            (navbar("combined"))

            main.container {
                // Wrap all components in a form to enable easy state sharing
                form id="page-state" {
                    div.grid {
                        div {
                            (counter(CounterState::default(), UrlBuilder::new("/counter", "").with_main_page("/combined"), cookies.clone(), query.clone()).await)
                            (greeter(GreeterState::default(), UrlBuilder::new("/greeter", "").with_main_page("/combined"), cookies.clone(), query.clone()).await)
                        }
                        div {
                            (user_table(UserTableState::default(), UrlBuilder::new("/user_table", "").with_main_page("/combined"), Extension(app_state), cookies, query, auth_for_component).await)
                        }
                    }
                }
            }
        }
    }
    .into();

    page.into_response()
}
