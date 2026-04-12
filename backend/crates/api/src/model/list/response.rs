use rc_log_application::model::list::model::ModelDto;
use rc_log_application::shared::pagination::PaginatedResult;
use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListResponse {
    pub items: Vec<ModelDto>,
    pub total: u64,
    pub page: u32,
    pub page_size: u32,
    pub total_pages: u64,
}

impl From<PaginatedResult<ModelDto>> for ListResponse {
    fn from(result: PaginatedResult<ModelDto>) -> Self {
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
