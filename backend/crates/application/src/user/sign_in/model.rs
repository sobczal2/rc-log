use rc_log_domain::user::User;
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct SignInInput {
    pub username: String,
    pub password: String,
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
        Self {
            id: Uuid::from(user.id()),
            username: user.username().as_str().to_string(),
            email: user.email().as_str().to_string(),
        }
    }
}
