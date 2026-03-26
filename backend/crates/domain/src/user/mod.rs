pub mod query;

use uuid::Uuid;

use crate::shared::password_hash::PasswordHash;

/// User aggregate root
///
/// Represents a user in the system with authentication credentials.
#[derive(Debug, Clone)]
pub struct User {
    id: Uuid,
    username: String,
    email: String,
    password_hash: PasswordHash,
}

impl User {
    /// Create a new user
    pub fn new(id: Uuid, username: String, email: String, password_hash: PasswordHash) -> Self {
        Self {
            id,
            username,
            email,
            password_hash,
        }
    }

    /// Get the user's ID
    pub fn id(&self) -> Uuid {
        self.id
    }

    /// Get the user's username
    pub fn username(&self) -> &str {
        &self.username
    }

    /// Get the user's email
    pub fn email(&self) -> &str {
        &self.email
    }

    /// Get the user's password hash
    pub fn password_hash(&self) -> &PasswordHash {
        &self.password_hash
    }
}
