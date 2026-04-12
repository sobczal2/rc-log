use rc_log_application::model::update::model::ModelDto;
use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateResponse {
    #[serde(flatten)]
    pub data: ModelDto,
}

impl From<ModelDto> for UpdateResponse {
    fn from(dto: ModelDto) -> Self {
        Self { data: dto }
    }
}
