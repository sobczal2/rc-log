use rc_log_application::maneuver::list::model::ManeuverDto;
use rc_log_application::shared::pagination::PaginatedResult;
use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListResponse {
    pub items: Vec<ManeuverDto>,
    pub total: u64,
    pub page: u32,
    pub page_size: u32,
    pub total_pages: u64,
}

impl From<PaginatedResult<ManeuverDto>> for ListResponse {
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
