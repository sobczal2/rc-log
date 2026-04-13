use rc_log_application::session::add_performed_variation::model::SessionDto;
use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AddPerformedVariationResponse {
    #[serde(flatten)]
    pub data: SessionDto,
}

impl From<SessionDto> for AddPerformedVariationResponse {
    fn from(dto: SessionDto) -> Self {
        Self { data: dto }
    }
}
