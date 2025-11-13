# htmoxide

A Rust framework for building interactive web applications with htmx.

## Overview

htmoxide combines Rust's type safety and performance with htmx's simplicity to create dynamic web applications without writing JavaScript. Components manage their state through URL query parameters, enabling bookmarkable, shareable application states.

## Features

- **Component Macro** - Auto-register routes and handle state extraction
- **URL State Management** - Component state serialized in URLs (bookmarkable/shareable)
- **UrlBuilder** - Automatic state merging across component interactions
- **Shared Application State** - Via Axum's Extension/Arc pattern
- **Type-safe** - Leverages Rust's type system and Serde for state
- **Server-side Rendering** - Components render on server, htmx handles DOM updates

## Quick Start

See the [dashboard example](examples/dashboard) for a complete demo.

```rust
use htmoxide::prelude::*;

#[derive(Deserialize, Serialize, Default)]
struct CounterState {
    count: i32,
}

#[component]
async fn counter(state: CounterState, url: UrlBuilder) -> Html {
    let increment_url = url.clone().with_params([("count", state.count + 1)]);
    
    html! {
        div id="counter" {
            h2 { "Count: " (state.count) }
            button hx-get=(increment_url.build()) 
                   hx-target="#counter" {
                "Increment"
            }
        }
    }.into()
}

#[tokio::main]
async fn main() {
    let app = app()
        .page("/", index_page);
    
    axum::serve(listener, app).await.unwrap();
}
```

## Project Structure

- `htmoxide/` - Core framework
- `htmoxide-macros/` - Procedural macros
- `examples/dashboard/` - Comprehensive example

## License

MIT
