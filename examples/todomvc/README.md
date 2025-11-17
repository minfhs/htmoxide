# TodoMVC - htmoxide

A TodoMVC implementation showcasing htmoxide's component-based architecture with htmx.

## Key Features

### Component Macro
Components are auto-registered routes with built-in state hydration:

```rust
#[component(prefix = "/todos", path = "/create", method = "POST")]
pub async fn create_todo(
    state: TodoState,      // Auto-hydrated from query params
    url: UrlBuilder,       // Injected URL builder with state
    Extension(db): Extension<TodoDb>,  // Standard Axum extractors
    Body(form): Body<Form<NewTodoForm>>,
) -> Html {
    // Your logic here
}
```

### Type-Safe Component References
Components generate PascalCase marker types for compile-time URL building:

```rust
// Component defined as `create_todo` → generates `CreateTodo` type
url.for_component(CreateTodo).build()
url.for_component(TodoContainer).with_params([("filter", "active")]).build()
```

### Zero JavaScript
All interactivity handled by htmx attributes - no custom JavaScript needed:

```rust
button hx-post=(url.for_component(ToggleTodo).with_path_param("id", todo.id).build())
       hx-target="#todo-container"
       hx-swap="innerHTML" {
    "Toggle"
}
```

### State Management
- **URL Parameters**: Component state serialized in query strings (bookmarkable)
- **Server-Side Storage**: Shared state via Axum's `Extension` layer
- **Optional Cookie Persistence**: Enable with `persist-state` feature

## Running

```bash
cargo run -p todomvc
```

Visit http://127.0.0.1:3000

## Architecture

- `main.rs` - App setup with in-memory database (`Arc<Mutex<TodoList>>`)
- `components.rs` - All CRUD operations as htmoxide components
- `pages.rs` - Full page renderer
- `todos.rs` - Business logic and data models
