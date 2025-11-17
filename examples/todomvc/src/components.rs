use crate::TodoDb;
use crate::todos::{Todo, TodoList as TodoListData};
use axum::Extension;
use axum::extract::{Form, Path};
use htmoxide::prelude::*;

// View state for the todo list (only filter in URL now)
#[derive(Deserialize, Serialize, Default, Clone, Debug)]
pub struct TodoViewState {
    #[serde(default)]
    pub filter: String, // "", "active", or "completed"
}

// Form data for creating a new todo
#[derive(Deserialize)]
pub struct NewTodoForm {
    pub title: String,
}

// Form data for editing a todo
#[derive(Deserialize)]
pub struct EditTodoForm {
    pub title: String,
}

// Form data for toggle all
#[derive(Deserialize)]
pub struct ToggleAllForm {
    pub completed: bool,
}

// Main todo list component (full wrapper)
#[component]
pub async fn todo_list(
    state: TodoViewState,
    url: UrlBuilder,
    Extension(db): Extension<TodoDb>,
) -> Html {
    Html::new(html! {
        section .todoapp {
            header .header {
                h1 { "todos" }
                form
                    hx-post=(url.clone().for_component(CreateTodo).build())
                    hx-target="#todo-container"
                    hx-swap="innerHTML"
                    hx-on--after-request="this.reset()" {
                    input .new-todo
                        placeholder="What needs to be done?"
                        name="title"
                        autofocus;
                }
            }

            (render_todo_container(&state, &url, &db))
        }
    })
}

// Just the todo container (for filter updates)
#[component(path = "/todo_container")]
pub async fn todo_container(
    state: TodoViewState,
    url: UrlBuilder,
    Extension(db): Extension<TodoDb>,
) -> Html {
    Html::new(render_todo_container(&state, &url, &db))
}

// Render just the todo container contents (for HTMX updates)
fn render_todo_container(state: &TodoViewState, url: &UrlBuilder, db: &TodoDb) -> Markup {
    let todos = db.lock().unwrap();
    let active_count = todos.active_count();
    let completed_count = todos.completed_count();
    let filtered_todos = todos.filtered(&state.filter);
    let all_completed = !todos.todos.is_empty() && active_count == 0;

    html! {
        div #todo-container {
            @if !todos.todos.is_empty() {
            section #todo-list .main {
                input #toggle-all .toggle-all
                    type="checkbox"
                    checked[all_completed]
                    hx-post=(url.clone().for_component(ToggleAll).build())
                    hx-target="#todo-container"
                    hx-swap="innerHTML"
                    hx-vals=(format!(r#"{{"completed":{}}}"#, !all_completed));
                label for="toggle-all" { "Mark all as complete" }

                ul .todo-list {
                    @for todo in filtered_todos {
                        (render_todo(todo, url))
                    }
                }
            }

            footer .footer {
                span .todo-count {
                    strong { (active_count) }
                    " "
                    @if active_count == 1 { "item" } @else { "items" }
                    " left"
                }

                ul .filters {
                    li {
                        a .{@if state.filter.is_empty() { "selected" }}
                            href=(url.clone().with_params([("filter", "")]).build_main_url())
                            hx-get=(url.clone().for_component(TodoContainer).with_params([("filter", "")]).build())
                            hx-target="#todo-container"
                            hx-swap="outerHTML"
                            hx-push-url=(url.clone().with_params([("filter", "")]).build_main_url()) {
                            "All"
                        }
                    }
                    li {
                        a .{@if state.filter == "active" { "selected" }}
                            href=(url.clone().with_params([("filter", "active")]).build_main_url())
                            hx-get=(url.clone().for_component(TodoContainer).with_params([("filter", "active")]).build())
                            hx-target="#todo-container"
                            hx-swap="outerHTML"
                            hx-push-url=(url.clone().with_params([("filter", "active")]).build_main_url()) {
                            "Active"
                        }
                    }
                    li {
                        a .{@if state.filter == "completed" { "selected" }}
                            href=(url.clone().with_params([("filter", "completed")]).build_main_url())
                            hx-get=(url.clone().for_component(TodoContainer).with_params([("filter", "completed")]).build())
                            hx-target="#todo-container"
                            hx-swap="outerHTML"
                            hx-push-url=(url.clone().with_params([("filter", "completed")]).build_main_url()) {
                            "Completed"
                        }
                    }
                }

                @if completed_count > 0 {
                    button .clear-completed
                        hx-post=(url.clone().for_component(ClearCompleted).build())
                        hx-target="#todo-container"
                        hx-swap="innerHTML" {
                        "Clear completed"
                    }
                }
            }
            }
        }
    }
}

fn render_todo(todo: &Todo, url: &UrlBuilder) -> Markup {
    let editing = todo.editing.unwrap_or(false);

    html! {
        li .{@if todo.completed { "completed" }} .{@if editing { "editing" }}
            data-id=(todo.id) {

            div .view {
                input .toggle
                    type="checkbox"
                    checked[todo.completed]
                    hx-post=(url.clone().for_component(ToggleTodo).with_path_param("id", todo.id).build())
                    hx-target="#todo-container"
                    hx-swap="innerHTML";

                label
                    hx-get=(url.clone().for_component(EditTodo).with_path_param("id", todo.id).build())
                    hx-target=(format!("[data-id='{}']", todo.id))
                    hx-swap="outerHTML" {
                    (todo.title)
                }

                button .destroy
                    hx-delete=(url.clone().for_component(DeleteTodo).with_path_param("id", todo.id).build())
                    hx-target="#todo-container"
                    hx-swap="innerHTML";
            }

            @if editing {
                form hx-post=(url.clone().for_component(UpdateTodo).with_path_param("id", todo.id).build())
                    hx-target=(format!("[data-id='{}']", todo.id))
                    hx-swap="outerHTML" {
                    input .edit
                        name="title"
                        value=(todo.title)
                        autofocus;
                }
            }
        }
    }
}

// Render a single todo item (used when editing)
fn render_single_todo(todo: &Todo, url: &UrlBuilder) -> Markup {
    render_todo(todo, url)
}

// Create a new todo
#[component(prefix = "/todos", path = "/create", method = "POST")]
pub async fn create_todo(
    state: TodoViewState,
    url: UrlBuilder,
    Extension(db): Extension<TodoDb>,
    Body(form): Body<Form<NewTodoForm>>,
) -> Html {
    let title = form.title.trim();
    if !title.is_empty() {
        db.lock().unwrap().add(title.to_string());
    }

    // Return just the container contents
    Html::new(render_todo_container(&state, &url, &db))
}

// Toggle a todo's completed status
#[component(prefix = "/todos", path = "/{id}/toggle", method = "POST")]
pub async fn toggle_todo(
    state: TodoViewState,
    url: UrlBuilder,
    Extension(db): Extension<TodoDb>,
    Path(id): Path<usize>,
) -> Html {
    db.lock().unwrap().toggle(id);
    Html::new(render_todo_container(&state, &url, &db))
}

// Delete a todo
#[component(prefix = "/todos", path = "/{id}", method = "DELETE")]
pub async fn delete_todo(
    state: TodoViewState,
    url: UrlBuilder,
    Extension(db): Extension<TodoDb>,
    Path(id): Path<usize>,
) -> Html {
    db.lock().unwrap().delete(id);
    Html::new(render_todo_container(&state, &url, &db))
}

// Start editing a todo
#[component(prefix = "/todos", path = "/{id}/edit")]
pub async fn edit_todo(
    state: TodoViewState,
    url: UrlBuilder,
    Extension(db): Extension<TodoDb>,
    Path(id): Path<usize>,
) -> Html {
    let mut todos = db.lock().unwrap();

    // Set the editing flag for this todo
    if let Some(todo) = todos.todos.iter_mut().find(|t| t.id == id) {
        todo.editing = Some(true);
    }

    // Find the todo and render just that item
    if let Some(todo) = todos.todos.iter().find(|t| t.id == id) {
        Html::new(render_single_todo(todo, &url))
    } else {
        Html::new(html! {})
    }
}

// Update a todo's title
#[component(prefix = "/todos", path = "/{id}/update", method = "POST")]
pub async fn update_todo(
    state: TodoViewState,
    url: UrlBuilder,
    Extension(db): Extension<TodoDb>,
    Path(id): Path<usize>,
    Body(form): Body<Form<EditTodoForm>>,
) -> Html {
    let title = form.title.trim();
    let mut todos = db.lock().unwrap();

    if !title.is_empty() {
        todos.update_title(id, title.to_string());
    }

    // Clear editing flag and render the updated todo
    if let Some(todo) = todos.todos.iter_mut().find(|t| t.id == id) {
        todo.editing = None;
    }

    if let Some(todo) = todos.todos.iter().find(|t| t.id == id) {
        Html::new(render_single_todo(todo, &url))
    } else {
        Html::new(html! {})
    }
}

// Toggle all todos
#[component(prefix = "/todos", path = "/toggle_all", method = "POST")]
pub async fn toggle_all(
    state: TodoViewState,
    url: UrlBuilder,
    Extension(db): Extension<TodoDb>,
    Body(form): Body<Form<ToggleAllForm>>,
) -> Html {
    db.lock().unwrap().toggle_all(form.completed);
    Html::new(render_todo_container(&state, &url, &db))
}

// Clear completed todos
#[component(prefix = "/todos", path = "/clear_completed", method = "POST")]
pub async fn clear_completed(
    state: TodoViewState,
    url: UrlBuilder,
    Extension(db): Extension<TodoDb>,
) -> Html {
    db.lock().unwrap().clear_completed();
    Html::new(render_todo_container(&state, &url, &db))
}
