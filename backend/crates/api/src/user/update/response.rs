use rc_log_application::user::update::model::UserDto;
use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateUserResponse {
    #[serde(flatten)]
    pub data: UserDto,
}

impl From<UserDto> for UpdateUserResponse {
    fn from(dto: UserDto) -> Self {
        Self { data: dto }
    }
}
