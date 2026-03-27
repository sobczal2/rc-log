use rc_log_application::user::sign_up::model::UserDto;
use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SignUpResponse {
    pub token: String,
    pub user: UserDto,
}
