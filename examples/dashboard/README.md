# htmoxide Dashboard Example

A comprehensive demo showcasing htmoxide's features for building interactive htmx applications.

## Features

- **Simple Components** - Counter and greeter with URL state management
- **Data Tables** - Sortable and filterable user table
- **Shared State** - Application-wide state with Extension/Arc pattern
- **Modular Structure** - Clean separation of components, pages, and layout

## Running

```bash
cargo run
```

Visit http://localhost:3000

## Structure

```
src/
├── main.rs              - Server setup
├── models.rs            - Data models
├── state.rs             - Application state
├── layout.rs            - Layout components (head, header, navbar, styles)
├── pages.rs             - Page handlers
└── components/          - Reusable htmx components
    ├── counter.rs
    ├── greeter.rs
    └── user_table.rs
```

## Key Concepts

- **Component State** - Each component has serializable state in URL query params
- **URL Builder** - Automatic state merging for component interactions  
- **App State** - Shared application state via `Extension<Arc<AppState>>`
- **Server-side Rendering** - Components render on server, htmx updates DOM
