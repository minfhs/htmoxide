use htmoxide::prelude::*;
use tower_cookies::Cookies;
use axum::extract::Query;

#[derive(Deserialize, Serialize, Default, Clone, Debug)]
pub struct CounterState {
    #[serde(default)]
    pub count: i32,
}

#[component]
pub async fn counter(
    state: CounterState,
    url: UrlBuilder,
    _cookies: Cookies,
    _query: Query<std::collections::HashMap<String, String>>,
) -> Html {
    // Build URLs with updated count, preserving all other parameters
    let increment_url = url.clone().with_params([("count", state.count + 1)]);
    let decrement_url = url.clone().with_params([("count", state.count - 1)]);
    let reset_url = url.clone().with_params([("count", 0)]);

    let markup = html! {
        article id="counter" {
            header {
                h3 { "Counter: " (state.count) }
            }
            
            button hx-get=(increment_url.build())
                   hx-target="#counter"
                   hx-swap="outerHTML swap:200ms"
                   title="Increment counter" {
                "Increment"
            }
            button hx-get=(decrement_url.build())
                   hx-target="#counter"
                   hx-swap="outerHTML swap:200ms"
                   class="secondary"
                   title="Decrement counter" {
                "Decrement"
            }
            button hx-get=(reset_url.build())
                   hx-target="#counter"
                   hx-swap="outerHTML swap:200ms"
                   class="outline"
                   title="Reset to zero" {
                "Reset"
            }
        }
    };

    Html::new(markup).with_push_url(url.build_main_url())
}
