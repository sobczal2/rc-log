#[derive(Debug, Clone, Copy)]
pub struct Pagination {
    pub page: u32,
    pub page_size: u32,
}

impl Pagination {
    pub fn new(page: u32, page_size: u32) -> Self {
        Self { page, page_size }
    }

    pub fn offset(&self) -> u64 {
        (self.page.saturating_sub(1) as u64) * self.page_size as u64
    }

    pub fn limit(&self) -> u64 {
        self.page_size as u64
    }
}
