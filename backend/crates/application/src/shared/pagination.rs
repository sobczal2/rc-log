use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginationDto {
    pub page: u32,
    pub page_size: u32,
}

impl From<PaginationDto> for rc_log_domain::shared::pagination::Pagination {
    fn from(dto: PaginationDto) -> Self {
        Self::new(dto.page, dto.page_size)
    }
}

#[derive(Debug, Clone)]
pub struct PaginatedResult<T> {
    pub items: Vec<T>,
    pub total: u64,
    pub page: u32,
    pub page_size: u32,
}

impl<T> PaginatedResult<T> {
    pub fn new(items: Vec<T>, total: u64, page: u32, page_size: u32) -> Self {
        Self { items, total, page, page_size }
    }

    pub fn total_pages(&self) -> u64 {
        self.total.div_ceil(self.page_size as u64)
    }
}
