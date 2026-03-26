use rc_log_domain::user::User;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::shared::validator::{Validate, ValidationError};

#[derive(Debug, Clone, Deserialize)]
pub struct GetUserByIdInput {
    pub id: Uuid,
}

impl Validate for GetUserByIdInput {
    fn validate(&self) -> Result<(), Vec<ValidationError>> {
        if self.id.is_nil() {
            return Err(vec![ValidationError::new("id", "must not be empty string (nil UUID)")]);
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserDto {
    pub id: Uuid,
    pub username: String,
    pub email: String,
}

impl From<User> for UserDto {
    fn from(user: User) -> Self {
        UserDto {
            id: user.id(),
            username: user.username().to_string(),
            email: user.email().to_string(),
        }
    }
}
