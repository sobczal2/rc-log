use rc_log_application::model::create::model::ModelDto;
use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateResponse {
    #[serde(flatten)]
    pub data: ModelDto,
}

impl From<ModelDto> for CreateResponse {
    fn from(dto: ModelDto) -> Self {
        Self { data: dto }
    }
}
