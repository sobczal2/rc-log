use rc_log_application::session::add_performed_variation::model::PerformedVariationDto;
use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AddPerformedVariationResponse {
    #[serde(flatten)]
    pub data: PerformedVariationDto,
}

impl From<PerformedVariationDto> for AddPerformedVariationResponse {
    fn from(dto: PerformedVariationDto) -> Self {
        Self { data: dto }
    }
}
