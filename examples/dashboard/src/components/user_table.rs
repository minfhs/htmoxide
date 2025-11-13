use htmoxide::prelude::*;
use htmoxide::{component, UrlBuilder};
use crate::state::AppStateExt;
use crate::auth::AuthSession;

#[derive(Deserialize, Serialize, Default, Clone, Debug)]
pub struct UserTableState {
    #[serde(default)]
    pub sort: String,  // "name", "email", "role", or ""
    #[serde(default)]
    pub filter: String,  // filter text
}

#[component]
pub async fn user_table(
    state: UserTableState, 
    url: UrlBuilder,
    auth_session: AuthSession,
    app_state: AppStateExt
) -> Html {
    // Require authentication for this component
    if auth_session.user.is_none() {
        return Html::new(html! {
            div.error {
                p { "Please " a href="/login" { "log in" } " to view the user table." }
            }
        });
    }
    
    let app_state = &**app_state;
    // Get users from shared state
    let mut users = app_state.users.lock().unwrap().clone();
    
    // Increment request counter
    {
        let mut count = app_state.request_count.lock().unwrap();
        *count += 1;
    }

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

    // Use base component URL - parameters will come from form inputs
    let component_path = "/user_table";
    
    
    // Build URLs for sort buttons, preserving filter
    let name_sort_url = url.clone().with_params([("sort", "name")]);
    let email_sort_url = url.clone().with_params([("sort", "email")]);
    let role_sort_url = url.clone().with_params([("sort", "role")]);
    
    // Get all params to preserve in filter form
    let all_params = url.all_params();
    
    let request_count = app_state.request_count.lock().unwrap();

    let markup = html! {
        article id="user-table" {
            header {
                h2 { "User Table" }
                p { 
                    small { 
                        "Request count: " (request_count) " | Users in database: " (app_state.users.lock().unwrap().len()) 
                    }
                }
            }

            form action=(component_path)
                 hx-get=(component_path)
                 hx-target="#user-table"
                 hx-swap="outerHTML"
                 hx-indicator="#search-indicator" {
                fieldset role="group" {
                    input type="text"
                           id="user-filter-input"
                           name="filter"
                           value=(state.filter)
                           placeholder="Filter users..."
                           hx-get=(component_path)
                           hx-trigger="keyup changed delay:500ms, search"
                           hx-target="#user-table"
                           hx-swap="outerHTML"
                           hx-include="closest form"
                           hx-sync="closest form:replace"
                           hx-vals="js:{filter: document.getElementById('user-filter-input').value}" // Need to force filter= param
                           aria-label="Filter users";
                    
                    // Include current sort as hidden field in the form
                    input type="hidden" name="sort" value=(state.sort);
                    
                    // Hidden inputs to preserve other components' state (like count, name)
                    @for (key, value) in all_params {
                        @if key != "filter" && key != "sort" && !value.is_empty() {
                            input type="hidden" name=(key) value=(value);
                        }
                    }
                    
                    // Loading indicator
                    span id="search-indicator" class="htmx-indicator" style="margin-left: 0.5rem;" {
                        "⏳"
                    }
                }
            }

            @if users.is_empty() && !state.filter.is_empty() {
                div style="text-align: center; padding: 2rem; color: var(--muted-color);" {
                    p { "🔍 No users found matching \"" (state.filter) "\"" }
                    p { 
                        small { "Try a different search term" }
                    }
                }
            } @else if users.is_empty() {
                div style="text-align: center; padding: 2rem; color: var(--muted-color);" {
                    p { "No users available" }
                }
            } @else {
                table {
                    thead {
                        tr {
                            th { "ID" }
                            th {
                                button hx-get=(name_sort_url.build())
                                       hx-target="#user-table"
                                       hx-swap="outerHTML"
                                       hx-indicator="#search-indicator"
                                       class="sort-button" {
                                    "Name " @if state.sort == "name" { "↓" } @else { "↕" }
                                }
                            }
                            th {
                                button hx-get=(email_sort_url.build())
                                       hx-target="#user-table"
                                       hx-swap="outerHTML"
                                       hx-indicator="#search-indicator"
                                       class="sort-button" {
                                    "Email " @if state.sort == "email" { "↓" } @else { "↕" }
                                }
                            }
                            th {
                                button hx-get=(role_sort_url.build())
                                       hx-target="#user-table"
                                       hx-swap="outerHTML"
                                       hx-indicator="#search-indicator"
                                       class="sort-button" {
                                    "Role " @if state.sort == "role" { "↓" } @else { "↕" }
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
        }
    };

    Html::new(markup).with_push_url(url.build_main_url())
}
