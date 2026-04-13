use rc_log_application::session::list::model::SessionDto;
use rc_log_application::shared::pagination::PaginatedResult;
use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListResponse {
    pub items: Vec<SessionDto>,
    pub total: u64,
    pub page: u32,
    pub page_size: u32,
    pub total_pages: u64,
}

impl From<PaginatedResult<SessionDto>> for ListResponse {
    fn from(result: PaginatedResult<SessionDto>) -> Self {
        Self {
            total_pages: result.total_pages(),
            items: result.items,
            total: result.total,
            page: result.page,
            page_size: result.page_size,
        }
    }
}
