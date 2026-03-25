use rc_log_application::maneuver::get_by_id::model::ManeuverDto as GetManeuverByIdDto;
use rc_log_application::maneuver::list::model::ManeuverDto as ListManeuversDto;
use rc_log_application::shared::pagination::PaginatedResult;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct GetManeuverByIdResponse {
    #[serde(flatten)]
    pub data: GetManeuverByIdDto,
}

impl From<GetManeuverByIdDto> for GetManeuverByIdResponse {
    fn from(m: GetManeuverByIdDto) -> Self {
        Self { data: m }
    }
}

#[derive(Debug, Serialize)]
pub struct ListManeuversResponse {
    pub items: Vec<ListManeuversDto>,
    pub total: u64,
    pub page: u32,
    pub page_size: u32,
    pub total_pages: u64,
}

impl From<PaginatedResult<ListManeuversDto>> for ListManeuversResponse {
    fn from(result: PaginatedResult<ListManeuversDto>) -> Self {
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
