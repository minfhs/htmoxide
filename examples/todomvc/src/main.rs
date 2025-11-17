use axum::routing::get;
use htmoxide::prelude::*;
use std::sync::{Arc, Mutex};

mod components;
mod pages;
mod todos;

use pages::index_page;
use todos::TodoList as TodoListData;

// In-memory database
pub type TodoDb = Arc<Mutex<TodoListData>>;

#[tokio::main]
async fn main() {
    // Create in-memory todo store
    let db = Arc::new(Mutex::new(TodoListData::default()));

    let app = htmoxide::app()
        .route("/", get(index_page))
        .layer(axum::Extension(db))
        .htmx();

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("TodoMVC running on http://127.0.0.1:3000");

    axum::serve(listener, app).await.unwrap();
}
