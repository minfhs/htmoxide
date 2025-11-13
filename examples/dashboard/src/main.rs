use htmoxide::prelude::*;

// ============================================================================
// Common Layout Components
// ============================================================================

fn header() -> Markup {
    html! {
        div.header {
            h1 { "htmoxide Demo" }
            p { "A Rust framework for building htmx applications" }
        }
    }
}

fn navbar(current_page: &str) -> Markup {
    html! {
        nav {
            a href="/simple" class=(if current_page == "simple" { "active" } else { "" }) {
                "Simple Demo"
            }
            " | "
            a href="/users" class=(if current_page == "users" { "active" } else { "" }) {
                "User Table"
            }
        }
    }
}

// ============================================================================
// Simple Demo Components (Counter & Greeter)
// ============================================================================

#[derive(Deserialize, Serialize, Default, Clone, Debug)]
struct CounterState {
    #[serde(default)]
    count: i32,
}

#[component]
async fn counter(state: CounterState, url: UrlBuilder) -> Html {
    let increment_url = url.clone().with_params([("count", state.count + 1)]);
    let decrement_url = url.clone().with_params([("count", state.count - 1)]);
    let reset_url = url.clone().with_params([("count", 0)]);

    let markup = html! {
        div id="counter" {
            // Hidden input so other components can include this value
            input type="hidden" name="count" value=(state.count);

            h2 { "Counter: " (state.count) }
            button hx-get=(increment_url.build())
                   hx-target="#counter"
                   hx-swap="outerHTML" {
                "Increment"
            }
            button hx-get=(decrement_url.build())
                   hx-target="#counter"
                   hx-swap="outerHTML" {
                "Decrement"
            }
            button hx-get=(reset_url.build())
                   hx-target="#counter"
                   hx-swap="outerHTML" {
                "Reset"
            }
        }
    };

    Html::new(markup).with_push_url(url.build_main_url())
}

// ============================================================================
// Simple greeting component
// ============================================================================

#[derive(Deserialize, Serialize, Default, Clone, Debug)]
struct GreeterState {
    #[serde(default)]
    name: String,
}

// ============================================================================
// User Table Components
// ============================================================================

#[derive(Clone, Debug)]
struct User {
    id: u32,
    name: String,
    email: String,
    role: String,
}

fn get_sample_users() -> Vec<User> {
    vec![
        User { id: 1, name: "Alice Johnson".to_string(), email: "alice@example.com".to_string(), role: "Admin".to_string() },
        User { id: 4, name: "Diana Prince".to_string(), email: "diana@example.com".to_string(), role: "Admin".to_string() },
        User { id: 6, name: "Fiona Green".to_string(), email: "fiona@example.com".to_string(), role: "Moderator".to_string() },
        User { id: 2, name: "Bob Smith".to_string(), email: "bob@example.com".to_string(), role: "User".to_string() },
        User { id: 3, name: "Charlie Brown".to_string(), email: "charlie@example.com".to_string(), role: "User".to_string() },
        User { id: 5, name: "Evan Davis".to_string(), email: "evan@example.com".to_string(), role: "User".to_string() },
    ]
}

#[derive(Deserialize, Serialize, Default, Clone, Debug)]
struct UserTableState {
    #[serde(default)]
    sort: String,  // "name", "email", "role", or ""
    #[serde(default)]
    filter: String,  // filter text
}

#[component]
async fn user_table(state: UserTableState, url: UrlBuilder) -> Html {
    let mut users = get_sample_users();

    // Apply filter
    if !state.filter.is_empty() {
        let filter_lower = state.filter.to_lowercase();
        users.retain(|u| {
            u.name.to_lowercase().contains(&filter_lower) ||
            u.email.to_lowercase().contains(&filter_lower) ||
            u.role.to_lowercase().contains(&filter_lower)
        });
    }

    // Apply sort
    match state.sort.as_str() {
        "name" => users.sort_by(|a, b| a.name.cmp(&b.name)),
        "email" => users.sort_by(|a, b| a.email.cmp(&b.email)),
        "role" => users.sort_by(|a, b| a.role.cmp(&b.role)),
        _ => {}
    }

    let name_sort_url = url.clone().with_params([("sort", "name")]);
    let email_sort_url = url.clone().with_params([("sort", "email")]);
    let role_sort_url = url.clone().with_params([("sort", "role")]);

    // For filter form, use base component URL without params - form inputs will provide all params
    let filter_url = UrlBuilder::new("/user_table", "").build();

    let markup = html! {
        div id="user-table" {
            // Only include filter in hidden inputs for sort buttons to pick up
            // Don't include sort as hidden input to avoid conflicts
            div id="user-table-filter-state" {
                input type="hidden" name="filter" value=(state.filter);
            }

            h2 { "User Table" }

            form hx-get=(filter_url)
                 hx-target="#user-table"
                 hx-swap="outerHTML"
                 hx-trigger="submit" {
                input type="text"
                       name="filter"
                       value=(state.filter)
                       placeholder="Filter users...";
                // Include current sort as hidden field in the form
                input type="hidden" name="sort" value=(state.sort);
                button type="submit" { "Filter" }
                @if !state.filter.is_empty() {
                    button hx-get=(url.clone().with_params([("filter", ""), ("sort", state.sort.as_str())]).build())
                           hx-target="#user-table"
                           hx-swap="outerHTML"
                           type="button" {
                        "Clear Filter"
                    }
                }
            }

            table {
                thead {
                    tr {
                        th { "ID" }
                        th {
                            button hx-get=(name_sort_url.build())
                                   hx-target="#user-table"
                                   hx-swap="outerHTML"
                                   hx-include="#user-table-filter-state"
                                   class="sort-button" {
                                "Name " @if state.sort == "name" { "↓" }
                            }
                        }
                        th {
                            button hx-get=(email_sort_url.build())
                                   hx-target="#user-table"
                                   hx-swap="outerHTML"
                                   hx-include="#user-table-filter-state"
                                   class="sort-button" {
                                "Email " @if state.sort == "email" { "↓" }
                            }
                        }
                        th {
                            button hx-get=(role_sort_url.build())
                                   hx-target="#user-table"
                                   hx-swap="outerHTML"
                                   hx-include="#user-table-filter-state"
                                   class="sort-button" {
                                "Role " @if state.sort == "role" { "↓" }
                            }
                        }
                    }
                }
                tbody {
                    @for user in users {
                        tr {
                            td { (user.id) }
                            td { (user.name) }
                            td { (user.email) }
                            td { (user.role) }
                        }
                    }
                }
            }
        }
    };

    Html::new(markup).with_push_url(url.build_main_url())
}

#[component]
async fn greeter(state: GreeterState, url: UrlBuilder) -> Html {
    let greeting = if state.name.is_empty() {
        "Hello, stranger!".to_string()
    } else {
        format!("Hello, {}!", state.name)
    };

    let markup = html! {
        div id="greeter" {
            h2 { (greeting) }
            form hx-get=(url.clone().build())
                 hx-target="#greeter"
                 hx-swap="outerHTML"
                 hx-include="#counter"
                 hx-trigger="submit" {
                input type="text" name="name" value=(state.name) placeholder="Enter your name";
                button type="submit" { "Greet" }
            }
        }
    };

    Html::new(markup).with_push_url(url.build_main_url())
}

// ============================================================================
// Page Routes
// ============================================================================

use axum::extract::Query;

fn common_styles() -> Markup {
    html! {
        style {
            r#"
            body {
                font-family: system-ui, -apple-system, sans-serif;
                max-width: 1000px;
                margin: 0 auto;
                padding: 2rem;
            }
            .header {
                margin-bottom: 1rem;
            }
            nav {
                margin: 1rem 0 2rem 0;
                padding: 1rem;
                background: #f5f5f5;
                border-radius: 8px;
            }
            nav a {
                text-decoration: none;
                color: #0066cc;
                padding: 0.5rem 1rem;
                border-radius: 4px;
            }
            nav a.active {
                background: #0066cc;
                color: white;
            }
            nav a:hover:not(.active) {
                background: #e0e0e0;
            }
            .components {
                display: grid;
                gap: 2rem;
                margin-top: 2rem;
            }
            #counter, #greeter, #user-table {
                border: 2px solid #ddd;
                padding: 1.5rem;
                border-radius: 8px;
            }
            button {
                margin: 0.5rem 0.5rem 0 0;
                padding: 0.5rem 1rem;
                font-size: 1rem;
                cursor: pointer;
                background: #fff;
                border: 1px solid #ddd;
                border-radius: 4px;
            }
            button:hover {
                background: #f5f5f5;
            }
            input[type="text"] {
                padding: 0.5rem;
                font-size: 1rem;
                margin-right: 0.5rem;
                border: 1px solid #ddd;
                border-radius: 4px;
            }
            table {
                width: 100%;
                border-collapse: collapse;
                margin-top: 1rem;
            }
            th, td {
                padding: 0.75rem;
                text-align: left;
                border-bottom: 1px solid #ddd;
            }
            th {
                background: #f5f5f5;
                font-weight: 600;
            }
            .sort-button {
                margin: 0;
                padding: 0;
                background: none;
                border: none;
                font-weight: 600;
                cursor: pointer;
            }
            .sort-button:hover {
                background: #e0e0e0;
            }
            "#
        }
    }
}

async fn index() -> Page {
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

async fn simple_page(Query(params): Query<std::collections::HashMap<String, String>>) -> Page {
    // Build query string from params
    let query_string = serde_urlencoded::to_string(&params).unwrap_or_default();

    // Extract state for each component
    let counter_state: CounterState = serde_urlencoded::from_str(&query_string).unwrap_or_default();
    let greeter_state: GreeterState = serde_urlencoded::from_str(&query_string).unwrap_or_default();

    // Create URL builders for components - they'll auto-detect page path from htmx headers
    let counter_url = UrlBuilder::new("/counter", &query_string).with_main_page("/simple");
    let greeter_url = UrlBuilder::new("/greeter", &query_string).with_main_page("/simple");

    html! {
        head {
            title { "Simple Demo - htmoxide" }
            script src="https://unpkg.com/htmx.org@1.9.10" {}
            (common_styles())
        }
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

async fn users_page(Query(params): Query<std::collections::HashMap<String, String>>) -> Page {
    // Build query string from params
    let query_string = serde_urlencoded::to_string(&params).unwrap_or_default();

    // Extract state for user table
    let user_table_state: UserTableState = serde_urlencoded::from_str(&query_string).unwrap_or_default();

    // Create URL builder for component with main page path
    let user_table_url = UrlBuilder::new("/user_table", &query_string).with_main_page("/users");

    html! {
        head {
            title { "User Table - htmoxide" }
            script src="https://unpkg.com/htmx.org@1.9.10" {}
            (common_styles())
        }
        body {
            (header())
            (navbar("users"))

            div.components {
                (user_table(user_table_state, user_table_url).await)
            }
        }
    }
    .into()
}

// ============================================================================
// App setup
// ============================================================================

#[tokio::main]
async fn main() {
    let app = app()
        .page("/", index)
        .page("/simple", simple_page)
        .page("/users", users_page)
        .build();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    println!("🚀 Server running on http://localhost:3000");
    println!("   - Main page: http://localhost:3000/");
    println!("   - Simple demo: http://localhost:3000/simple");
    println!("   - User table: http://localhost:3000/users");
    axum::serve(listener, app).await.unwrap();
}
