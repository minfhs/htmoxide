use axum_login::{AuthUser, AuthnBackend, UserId};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::sync::RwLock;
use std::sync::Arc;

/// User type for authentication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub password_hash: String, // In production, use proper password hashing
    pub name: String,
}

impl AuthUser for User {
    type Id = i64;

    fn id(&self) -> Self::Id {
        self.id
    }

    fn session_auth_hash(&self) -> &[u8] {
        self.password_hash.as_bytes()
    }
}

/// Simple in-memory user store
#[derive(Debug, Clone, Default)]
pub struct UserStore {
    users: Arc<RwLock<HashMap<String, User>>>,
}

impl UserStore {
    pub fn new() -> Self {
        let mut users = HashMap::new();
        
        // Add demo users (password is just plain text for demo - use bcrypt in production!)
        users.insert(
            "admin".to_string(),
            User {
                id: 1,
                username: "admin".to_string(),
                password_hash: "admin123".to_string(), // Use bcrypt in production!
                name: "Admin User".to_string(),
            },
        );
        users.insert(
            "user".to_string(),
            User {
                id: 2,
                username: "user".to_string(),
                password_hash: "user123".to_string(), // Use bcrypt in production!
                name: "Regular User".to_string(),
            },
        );

        Self {
            users: Arc::new(RwLock::new(users)),
        }
    }

    pub async fn get_user(&self, username: &str) -> Option<User> {
        self.users.read().await.get(username).cloned()
    }
}

/// Credentials for login
#[derive(Debug, Clone, Deserialize)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}

/// Authentication backend
#[derive(Debug, Clone)]
pub struct Backend {
    user_store: UserStore,
}

impl Backend {
    pub fn new(user_store: UserStore) -> Self {
        Self { user_store }
    }
}

impl AuthnBackend for Backend {
    type User = User;
    type Credentials = Credentials;
    type Error = std::convert::Infallible;

    async fn authenticate(
        &self,
        creds: Self::Credentials,
    ) -> Result<Option<Self::User>, Self::Error> {
        let user = self.user_store.get_user(&creds.username).await;
        
        // In production, use bcrypt::verify or similar!
        Ok(user.filter(|u| u.password_hash == creds.password))
    }

    async fn get_user(&self, user_id: &UserId<Self>) -> Result<Option<Self::User>, Self::Error> {
        Ok(self
            .user_store
            .users
            .read()
            .await
            .values()
            .find(|u| u.id == *user_id)
            .cloned())
    }
}

pub type AuthSession = axum_login::AuthSession<Backend>;
