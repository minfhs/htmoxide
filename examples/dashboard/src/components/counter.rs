use htmoxide::prelude::*;

#[derive(Deserialize, Serialize, Default, Clone, Debug)]
pub struct CounterState {
    #[serde(default)]
    pub count: i32,
}

#[component]
pub async fn counter(state: CounterState, url: UrlBuilder) -> Html {
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
                   hx-swap="outerHTML" {
                "Increment"
            }
            button hx-get=(decrement_url.build())
                   hx-target="#counter"
                   hx-swap="outerHTML"
                   class="secondary" {
                "Decrement"
            }
            button hx-get=(reset_url.build())
                   hx-target="#counter"
                   hx-swap="outerHTML"
                   class="outline" {
                "Reset"
            }
        }
    };

    Html::new(markup).with_push_url(url.build_main_url())
}
