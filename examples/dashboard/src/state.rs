use std::sync::{Arc, Mutex};
use axum::Extension;
use crate::models::User;

/// Shared application state that persists across requests
#[derive(Clone)]
pub struct AppState {
    /// In-memory user database
    pub users: Arc<Mutex<Vec<User>>>,
    /// Global request counter
    pub request_count: Arc<Mutex<u64>>,
}

/// Type alias for extracting app state in components
/// Use this in #[component] functions to avoid syntax issues with generics
pub type AppStateExt = Extension<Arc<AppState>>;

impl AppState {
    pub fn new() -> Self {
        Self {
            users: Arc::new(Mutex::new(get_initial_users())),
            request_count: Arc::new(Mutex::new(0)),
        }
    }
}

fn get_initial_users() -> Vec<User> {
    vec![
        User { id: 1, name: "Alice Johnson".to_string(), email: "alice@example.com".to_string(), role: "Admin".to_string() },
        User { id: 4, name: "Diana Prince".to_string(), email: "diana@example.com".to_string(), role: "Admin".to_string() },
        User { id: 6, name: "Fiona Green".to_string(), email: "fiona@example.com".to_string(), role: "Moderator".to_string() },
        User { id: 2, name: "Bob Smith".to_string(), email: "bob@example.com".to_string(), role: "User".to_string() },
        User { id: 3, name: "Charlie Brown".to_string(), email: "charlie@example.com".to_string(), role: "User".to_string() },
        User { id: 5, name: "Evan Davis".to_string(), email: "evan@example.com".to_string(), role: "User".to_string() },
    ]
}
