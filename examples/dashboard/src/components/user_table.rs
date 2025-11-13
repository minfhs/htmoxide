use htmoxide::prelude::*;
use crate::state::AppStateExt;

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
    app_state: AppStateExt
) -> Html {
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

    let name_sort_url = url.clone().with_params([("sort", "name")]);
    let email_sort_url = url.clone().with_params([("sort", "email")]);
    let role_sort_url = url.clone().with_params([("sort", "role")]);

    // For filter form, use base component URL without params - form inputs will provide all params
    let filter_url = UrlBuilder::new("/user_table", "").build();
    
    let request_count = app_state.request_count.lock().unwrap();

    let markup = html! {
        div id="user-table" {
            // Only include filter in hidden inputs for sort buttons to pick up
            // Don't include sort as hidden input to avoid conflicts
            div id="user-table-filter-state" {
                input type="hidden" name="filter" value=(state.filter);
            }

            h2 { "User Table" }
            p.text-muted { "Request count: " (request_count) " | Users in database: " (app_state.users.lock().unwrap().len()) }

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
