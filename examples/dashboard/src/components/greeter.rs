use htmoxide::prelude::*;

#[derive(Deserialize, Serialize, Default, Clone, Debug)]
pub struct GreeterState {
    #[serde(default)]
    pub name: String,
}

#[component]
pub async fn greeter(state: GreeterState, url: UrlBuilder) -> Html {
    let greeting = if state.name.is_empty() {
        "Hello, stranger!".to_string()
    } else {
        format!("Hello, {}!", state.name)
    };

    let markup = html! {
        article id="greeter" {
            // Hidden input so other components can include this value
            input type="hidden" name="name" value=(state.name);
            
            header {
                h3 { (greeting) }
            }
            form hx-get=(url.clone().build())
                 hx-target="#greeter"
                 hx-swap="outerHTML"
                 hx-include="#counter, #user-table-filter-state"
                 hx-trigger="submit" {
                fieldset role="group" {
                    input type="text" name="name" value=(state.name) placeholder="Enter your name";
                    button type="submit" { "Greet" }
                }
            }
        }
    };

    Html::new(markup).with_push_url(url.build_main_url())
}
