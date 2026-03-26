pub mod query;
pub mod username;

use uuid::Uuid;

use crate::shared::email::Email;
use crate::shared::password_hash::PasswordHash;
use username::Username;

/// User aggregate root
///
/// Represents a user in the system with authentication credentials.
#[derive(Debug, Clone)]
pub struct User {
    id: Uuid,
    username: Username,
    email: Email,
    password_hash: PasswordHash,
}

impl User {
    pub fn new(id: Uuid, username: Username, email: Email, password_hash: PasswordHash) -> Self {
        Self { id, username, email, password_hash }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn username(&self) -> &Username {
        &self.username
    }

    pub fn email(&self) -> &Email {
        &self.email
    }

    pub fn password_hash(&self) -> &PasswordHash {
        &self.password_hash
    }
}
