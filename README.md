# HTMoXide

Build interactive web apps with htmx and Axum - zero JavaScript required.

## Overview

htmoxide is a thin layer over Axum that adds component-based routing and automatic state management. You get all of Axum's extractors and middleware, plus conventions that make htmx development ergonomic.

## What htmoxide Adds to Axum

### 1. Component Macro - Auto-Route Registration
Define handlers as components with automatic route registration:

```rust
#[component(prefix = "/todos", path = "/{id}/toggle", method = "POST")]
pub async fn toggle_todo(
    state: TodoState,           // Auto-hydrated from query params
    url: UrlBuilder,            // Type-safe URL builder
    Extension(db): Extension<TodoDb>,  // Standard Axum extractors work
    Path(id): Path<usize>,
) -> Html { /* ... */ }
```

No manual `.route()` calls - components register themselves at compile time.

### 2. Type-Safe Component URLs
Components generate marker types for compile-time URL building:

```rust
// `toggle_todo` generates `ToggleTodo` type
url.for_component(ToggleTodo)
   .with_path_param("id", 5)
   .with_params([("filter", "active")])
   .build()  // "/todos/5/toggle?filter=active"
```

### 3. Automatic State Hydration
Component state deserializes from query params (and optionally cookies):

```rust
#[derive(Deserialize, Serialize, Default)]
struct TodoState {
    filter: String,  // Automatically populated from ?filter=active
}
```

### 4. Advanced Form Handling (Optional)
With the `qs-forms` feature, handle complex forms with array fields:

```rust
#[derive(Deserialize, Serialize, Default)]
struct CreatePost {
    title: String,
    tags: Vec<String>,  // Parses from tags[]=foo&tags[]=bar
}

#[component(method = "POST")]
async fn create_post(
    state: AppState,
    url: UrlBuilder,
    Body(QsForm(form)): Body<QsForm<CreatePost>>,
) -> Html {
    // form.tags contains ["foo", "bar"]
    /* ... */
}
```

Enable with: `htmoxide = { version = "0.1", features = ["qs-forms"] }`

## Everything Else is Axum

- Use any Axum extractor (`Extension`, `State`, `Path`, `Form`, `Json`, etc.)
- All Axum middleware works unchanged
- Standard `tower` and `tower-http` layers
- Same async runtime (tokio)

## Quick Start

```rust
use htmoxide::prelude::*;

#[derive(Deserialize, Serialize, Default)]
struct CounterState {
    count: i32,
}

#[component]
async fn counter(state: CounterState, url: UrlBuilder) -> Html {
    html! {
        div {
            p { "Count: " (state.count) }
            button hx-get=(url.with_params([("count", state.count + 1)]).build())
                   hx-target="closest div" {
                "Increment"
            }
        }
    }.into()
}

#[tokio::main]
async fn main() {
    let app = htmoxide::app()  // Components auto-register
        .route("/", get(index_page))
        .htmx();  // Add htmx middleware

    axum::serve(
        tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap(),
        app
    ).await.unwrap();
}
```

See the [TodoMVC example](examples/todomvc) for a complete application.

## Project Structure

- `htmoxide/` - Core framework
- `htmoxide-macros/` - Procedural macros
- `examples/todomvc/` - TodoMVC implementation example

## Built upon

- [axum](https://github.com/tokio-rs/axum)
- [maud](https://github.com/lambda-fairy/maud)
- [tower](https://github.com/tower-rs/tower)
- [tokio](https://github.com/tokio-rs/tokio)
- [serde](https://github.com/serde-rs/serde)
- and more

## License

MIT
