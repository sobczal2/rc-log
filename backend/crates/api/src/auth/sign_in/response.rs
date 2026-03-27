use rc_log_application::user::sign_in::model::UserDto;
use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SignInResponse {
    pub token: String,
    pub user: UserDto,
}
