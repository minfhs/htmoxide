use htmoxide::prelude::*;

#[derive(Deserialize, Serialize, Default, Clone, Debug)]
pub struct CounterState {
    #[serde(default)]
    pub count: i32,
}

#[component]
pub async fn counter(state: CounterState, url: UrlBuilder) -> Html {
    let increment_url = url.clone().with_params([("count", state.count + 1)]);
    let decrement_url = url.clone().with_params([("count", state.count - 1)]);
    let reset_url = url.clone().with_params([("count", 0)]);

    let markup = html! {
        article id="counter" {
            // Hidden input so other components can include this value
            input type="hidden" name="count" value=(state.count);

            header {
                h3 { "Counter: " (state.count) }
            }
            button hx-get=(increment_url.build())
                   hx-target="#counter"
                   hx-swap="outerHTML"
                   hx-include="#greeter, #user-table-filter-state" {
                "Increment"
            }
            button hx-get=(decrement_url.build())
                   hx-target="#counter"
                   hx-swap="outerHTML"
                   hx-include="#greeter, #user-table-filter-state"
                   class="secondary" {
                "Decrement"
            }
            button hx-get=(reset_url.build())
                   hx-target="#counter"
                   hx-swap="outerHTML"
                   hx-include="#greeter, #user-table-filter-state"
                   class="outline" {
                "Reset"
            }
        }
    };

    Html::new(markup).with_push_url(url.build_main_url())
}
