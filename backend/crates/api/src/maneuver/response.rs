use rc_log_application::maneuver::model::ManeuverDto;
use rc_log_application::shared::paginated_result::PaginatedResult;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct GetManeuverByIdResponse {
    #[serde(flatten)]
    pub data: ManeuverDto,
}

impl From<ManeuverDto> for GetManeuverByIdResponse {
    fn from(m: ManeuverDto) -> Self {
        Self { data: m }
    }
}

#[derive(Debug, Serialize)]
pub struct ListManeuversResponse {
    pub items: Vec<ManeuverDto>,
    pub total: u64,
    pub page: u32,
    pub page_size: u32,
    pub total_pages: u64,
}

impl From<PaginatedResult<ManeuverDto>> for ListManeuversResponse {
    fn from(result: PaginatedResult<ManeuverDto>) -> Self {
        let total_pages = result.total_pages();
        Self {
            items: result.items,
            total: result.total,
            page: result.page,
            page_size: result.page_size,
            total_pages,
        }
    }
}
