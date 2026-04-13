use rc_log_application::session::remove_performed_variation::model::SessionDto;
use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RemovePerformedVariationResponse {
    #[serde(flatten)]
    pub data: SessionDto,
}

impl From<SessionDto> for RemovePerformedVariationResponse {
    fn from(dto: SessionDto) -> Self {
        Self { data: dto }
    }
}
