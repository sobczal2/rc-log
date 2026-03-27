use rc_log_application::user::get_by_id::model::UserDto;
use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetByIdResponse {
    #[serde(flatten)]
    pub data: UserDto,
}

impl From<UserDto> for GetByIdResponse {
    fn from(u: UserDto) -> Self {
        Self { data: u }
    }
}
