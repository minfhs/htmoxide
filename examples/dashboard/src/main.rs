use htmoxide::prelude::*;
use std::sync::Arc;
use axum::Extension;
use axum::routing::get;
use tower_sessions::{Expiry, SessionManagerLayer};
use tower_sessions::cookie::time::Duration;
use tower_sessions::MemoryStore;
use axum_login::AuthManagerLayerBuilder;
use tower_cookies::CookieManagerLayer;

mod models;
mod state;
mod layout;
mod components;
mod pages;
mod auth;
mod auth_pages;

use state::AppState;
use pages::{index, simple_page, users_page, combined_page};
use auth::{Backend, UserStore};
use auth_pages::{login_page, login_handler, logout_handler};

#[tokio::main]
async fn main() {
    // Create shared application state
    let app_state = Arc::new(AppState::new());
    
    // Set up authentication
    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store)
        .with_expiry(Expiry::OnInactivity(Duration::days(1)));
    
    let user_store = UserStore::new();
    let backend = Backend::new(user_store);
    let auth_layer = AuthManagerLayerBuilder::new(backend, session_layer).build();
    
    let app = app()
        .route("/login", get(login_page).post(login_handler))
        .route("/logout", get(logout_handler))
        .route("/", get(index))
        .route("/simple", get(simple_page))
        .route("/users", get(users_page))
        .route("/combined", get(combined_page))
        .layer(Extension(app_state))
        .layer(auth_layer)
        .layer(CookieManagerLayer::new());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    println!("🚀 Server running on http://localhost:3000");
    println!("   - Login: http://localhost:3000/login");
    println!("   - Main page: http://localhost:3000/");
    println!("   - Simple demo: http://localhost:3000/simple");
    println!("   - User table: http://localhost:3000/users");
    println!("   - Combined view: http://localhost:3000/combined");
    println!("\n   Demo credentials: admin/admin123 or user/user123");
    axum::serve(listener, app).await.unwrap();
}
