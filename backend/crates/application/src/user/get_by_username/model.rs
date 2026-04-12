use rc_log_domain::user::User;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize)]
pub struct GetUserByUsernameInput {
    pub username: String,
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
            id: Uuid::from(user.id()),
            username: user.username().as_str().to_string(),
            email: user.email().as_str().to_string(),
        }
    }
}
