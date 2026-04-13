use rc_log_application::user::update_photo::model::UserDto;
use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateUserPhotoResponse {
    #[serde(flatten)]
    pub data: UserDto,
}

impl From<UserDto> for UpdateUserPhotoResponse {
    fn from(dto: UserDto) -> Self {
        Self { data: dto }
    }
}
