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

    // Use just the component path without params - form inputs will provide params
    let component_path = "/greeter";
    
    // Get all other params to preserve them in hidden inputs
    let all_params = url.all_params();
    
    let markup = html! {
        article id="greeter" {
            header {
                h3 { (greeting) }
            }
            div {
                input type="text" id="greeter-input" name="name" value=(state.name) placeholder="Enter your name" aria-label="Your name";
                
                // Hidden inputs to preserve other components' state
                @for (key, value) in all_params {
                    @if key != "name" && !value.is_empty() {
                        input type="hidden" name=(key) value=(value);
                    }
                }
                
                button hx-get=(component_path)
                       hx-include="closest div"
                       hx-target="#greeter"
                       hx-swap="outerHTML swap:200ms"
                       title="Update greeting" {
                    "Greet"
                }
            }
        }
    };

    Html::new(markup).with_push_url(url.build_main_url())
}
