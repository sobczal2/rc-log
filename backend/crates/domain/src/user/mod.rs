pub mod id;
pub mod query;
pub mod username;

use crate::asset::photo::PhotoId;
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
    photo_asset_id: Option<PhotoId>,
}

impl User {
    pub fn new(
        id: UserId,
        username: Username,
        email: Email,
        password_hash: PasswordHash,
        photo_asset_id: Option<PhotoId>,
    ) -> Self {
        Self { id, username, email, password_hash, photo_asset_id }
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

    pub fn photo_asset_id(&self) -> Option<&PhotoId> {
        self.photo_asset_id.as_ref()
    }
}
