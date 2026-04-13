use rc_log_application::model::update_photo::model::ModelDto;
use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdatePhotoResponse {
    #[serde(flatten)]
    pub data: ModelDto,
}

impl From<ModelDto> for UpdatePhotoResponse {
    fn from(dto: ModelDto) -> Self {
        Self { data: dto }
    }
}
