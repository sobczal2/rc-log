use rc_log_domain::user::User;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::shared::validator::{Validate, ValidationError};

#[derive(Debug, Clone, Deserialize)]
pub struct CreateUserInput {
    pub username: String,
    pub email: String,
    pub password_hash: String,
}

impl Validate for CreateUserInput {
    fn validate(&self) -> Result<(), Vec<ValidationError>> {
        let mut errors = Vec::new();

        if self.username.trim().is_empty() {
            errors.push(ValidationError::new("username", "must not be empty"));
        }

        if self.username.len() > 255 {
            errors.push(ValidationError::new("username", "must not exceed 255 characters"));
        }

        if self.email.trim().is_empty() {
            errors.push(ValidationError::new("email", "must not be empty"));
        }

        if self.email.len() > 255 {
            errors.push(ValidationError::new("email", "must not exceed 255 characters"));
        }

        if self.password_hash.trim().is_empty() {
            errors.push(ValidationError::new(
                "password_hash",
                "must not be empty",
            ));
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
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
