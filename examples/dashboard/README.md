# htmoxide Dashboard Example

A comprehensive demo showcasing htmoxide's features for building interactive htmx applications.

## Features

- **Simple Components** - Counter and greeter with URL state management
- **Data Tables** - Sortable and filterable user table with debounced search
- **Auth Protection** - Declarative component protection via `AuthSession` parameter
- **Cookie Persistence** - State persists across page loads via cookies
- **UX Polish** - Loading indicators, debounced inputs, smooth transitions
- **Client Helpers** - Cookie management and parameter preservation utilities
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
- **Cookie Fallback** - State persists via cookies when URL params are absent
- **URL Builder** - Automatic state merging for component interactions
- **Declarative Auth** - Add `_auth_session: AuthSession` parameter to auto-protect components
- **Client Helpers** - Use `cookie_cleaner_script()`, `preserve_params()`, `clear_input_handler()`
- **App State** - Shared application state via `Extension<Arc<AppState>>`
- **Server-side Rendering** - Components render on server, htmx updates DOM

## Authentication

The user table demonstrates automatic auth protection:

```rust
#[component]
async fn user_table(
    _auth_session: AuthSession,  // This parameter triggers auto-protection
    state: UserTableState,
    url: UrlBuilder,
) -> Html {
    // Component is automatically protected - redirects if not authenticated
}
```

## State Management

State priority: **URL params** → **Cookies** → **Defaults**

- URL params always win (bookmarkable/shareable)
- Cookies provide persistence across page loads
- Empty values automatically clear cookies via client-side JavaScript
