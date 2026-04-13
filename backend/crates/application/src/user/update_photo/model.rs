use rc_log_domain::user::User;
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug)]
pub struct UpdateUserPhotoInput {
    pub user_id: Uuid,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserDto {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub photo_asset_name: Option<String>,
}

impl From<User> for UserDto {
    fn from(user: User) -> Self {
        Self {
            id: Uuid::from(user.id()),
            username: user.username().as_str().to_string(),
            email: user.email().as_str().to_string(),
            photo_asset_name: user.photo_asset_name().map(|n| n.as_str().to_string()),
        }
    }
}
