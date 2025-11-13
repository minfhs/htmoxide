use htmoxide::prelude::*;
use htmoxide::RouterExt;
use std::sync::Arc;
use axum::Extension;

mod models;
mod state;
mod layout;
mod components;
mod pages;

use state::AppState;
use pages::{index, simple_page, users_page};

#[tokio::main]
async fn main() {
    // Create shared application state
    let app_state = Arc::new(AppState::new());
    
    let app = app()
        .page("/", index)
        .page("/simple", simple_page)
        .page("/users", users_page)
        .layer(Extension(app_state));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    println!("🚀 Server running on http://localhost:3000");
    println!("   - Main page: http://localhost:3000/");
    println!("   - Simple demo: http://localhost:3000/simple");
    println!("   - User table: http://localhost:3000/users");
    axum::serve(listener, app).await.unwrap();
}
