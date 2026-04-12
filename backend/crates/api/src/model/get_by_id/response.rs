use rc_log_application::model::get_by_id::model::ModelDto;
use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetByIdResponse {
    #[serde(flatten)]
    pub data: ModelDto,
}

impl From<ModelDto> for GetByIdResponse {
    fn from(dto: ModelDto) -> Self {
        Self { data: dto }
    }
}
