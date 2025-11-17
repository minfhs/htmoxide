use crate::TodoDb;
use crate::components::{TodoViewState, todo_list};
use axum::Extension;
use htmoxide::Page;
use htmoxide::prelude::*;

pub async fn index_page(Extension(db): Extension<TodoDb>) -> Page {
    let view_state = TodoViewState::default();
    let todo_list_url = UrlBuilder::new("/todo_list", "");

    html! {
        (maud::DOCTYPE)
        html lang="en" {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1";
                title { "TodoMVC • htmoxide" }
                link rel="stylesheet" href="https://unpkg.com/todomvc-common@1.0.5/base.css";
                link rel="stylesheet" href="https://unpkg.com/todomvc-app-css@2.4.2/index.css";
                script src="https://unpkg.com/htmx.org@2.0.3" {}
            }
            body {
                (todo_list(view_state, todo_list_url, Extension(db)).await)

                footer.info {
                    p { "Double-click to edit a todo" }
                    p { "Created with " a href="https://github.com/minfhs/htmoxide" { "htmoxide" } }
                    p { "Part of " a href="http://todomvc.com" { "TodoMVC" } }
                }
            }
        }
    }
    .into()
}
