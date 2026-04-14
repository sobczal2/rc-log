pub mod id;
pub mod query;
pub mod username;

use crate::asset::name::Name;
use crate::shared::email::Email;
use crate::shared::password_hash::PasswordHash;
use id::UserId;
use username::Username;

/// User aggregate root
///
/// Represents a user in the system with authentication credentials.
#[derive(Debug, Clone)]
pub struct User {
    id: UserId,
    username: Username,
    email: Email,
    password_hash: PasswordHash,
    photo_asset_name: Option<Name>,
}

impl User {
    pub fn new(
        id: UserId,
        username: Username,
        email: Email,
        password_hash: PasswordHash,
        photo_asset_name: Option<Name>,
    ) -> Self {
        Self { id, username, email, password_hash, photo_asset_name }
    }

    pub fn id(&self) -> UserId {
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

    pub fn photo_asset_name(&self) -> Option<&Name> {
        self.photo_asset_name.as_ref()
    }
}
