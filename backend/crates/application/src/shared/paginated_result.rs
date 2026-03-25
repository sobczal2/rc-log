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
