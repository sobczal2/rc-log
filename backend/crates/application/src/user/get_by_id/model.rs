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
            username: user.username().as_str().to_string(),
            email: user.email().as_str().to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use crate::shared::validator::Validate;

    use super::GetUserByIdInput;

    #[test]
    fn non_nil_uuid_passes_validation() {
        let input = GetUserByIdInput { id: Uuid::new_v4() };
        assert!(input.validate().is_ok());
    }

    #[test]
    fn nil_uuid_fails_validation() {
        let input = GetUserByIdInput { id: Uuid::nil() };
        let errs = input.validate().unwrap_err();
        assert_eq!(errs.len(), 1);
        assert_eq!(errs[0].field, "id");
    }
}
