use htmoxide::prelude::*;

// ============================================================================
// Simple counter component
// ============================================================================

#[derive(Deserialize, Serialize, Default, Clone, Debug)]
struct CounterState {
    #[serde(default)]
    count: i32,
}

#[component]
async fn counter(state: CounterState) -> Html {
    html! {
        div hx-get="/counter" hx-swap="outerHTML" {
            h2 { "Counter: " (state.count) }
            button hx-get={"/counter?count=" (state.count + 1)} {
                "Increment"
            }
            button hx-get={"/counter?count=" (state.count - 1)} {
                "Decrement"
            }
            button hx-get="/counter?count=0" {
                "Reset"
            }
        }
    }
    .into()
}

// ============================================================================
// Simple greeting component
// ============================================================================

#[derive(Deserialize, Serialize, Default, Clone, Debug)]
struct GreeterState {
    #[serde(default)]
    name: String,
}

#[component]
async fn greeter(state: GreeterState) -> Html {
    let greeting = if state.name.is_empty() {
        "Hello, stranger!".to_string()
    } else {
        format!("Hello, {}!", state.name)
    };

    html! {
        div hx-get="/greeter" hx-swap="outerHTML" {
            h2 { (greeting) }
            form hx-get="/greeter" hx-trigger="submit" {
                input type="text" name="name" value=(state.name) placeholder="Enter your name";
                button type="submit" { "Greet" }
            }
        }
    }
    .into()
}

// ============================================================================
// Main page
// ============================================================================

async fn index() -> Page {
    html! {
        head {
            title { "htmoxide Demo" }
            script src="https://unpkg.com/htmx.org@1.9.10" {}
            style {
                r#"
                body {
                    font-family: system-ui, -apple-system, sans-serif;
                    max-width: 800px;
                    margin: 0 auto;
                    padding: 2rem;
                }
                .components {
                    display: grid;
                    gap: 2rem;
                    margin-top: 2rem;
                }
                div[hx-get] {
                    border: 2px solid #ddd;
                    padding: 1.5rem;
                    border-radius: 8px;
                }
                button {
                    margin: 0.5rem 0.5rem 0 0;
                    padding: 0.5rem 1rem;
                    font-size: 1rem;
                    cursor: pointer;
                }
                input {
                    padding: 0.5rem;
                    font-size: 1rem;
                    margin-right: 0.5rem;
                }
                "#
            }
        }
        body {
            h1 { "htmoxide Demo" }
            p { "A Rust framework for building htmx applications" }

            div.components {
                (counter(CounterState::default()).await)
                (greeter(GreeterState::default()).await)
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
        .build();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    println!("🚀 Server running on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}
