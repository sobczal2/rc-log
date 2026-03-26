use rc_log_application::maneuver::get_by_id::model::ManeuverDto;
use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetByIdResponse {
    #[serde(flatten)]
    pub data: ManeuverDto,
}

impl From<ManeuverDto> for GetByIdResponse {
    fn from(m: ManeuverDto) -> Self {
        Self { data: m }
    }
}
