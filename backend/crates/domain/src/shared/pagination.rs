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

#[cfg(test)]
mod tests {
    use super::Pagination;

    #[test]
    fn first_page_offset_is_zero() {
        let p = Pagination::new(1, 20);
        assert_eq!(p.offset(), 0);
    }

    #[test]
    fn second_page_offset() {
        let p = Pagination::new(2, 20);
        assert_eq!(p.offset(), 20);
    }

    #[test]
    fn third_page_offset_with_small_page_size() {
        let p = Pagination::new(3, 10);
        assert_eq!(p.offset(), 20);
    }

    #[test]
    fn limit_equals_page_size() {
        let p = Pagination::new(5, 42);
        assert_eq!(p.limit(), 42);
    }

    #[test]
    fn page_zero_saturates_to_zero_offset() {
        // page=0 → saturating_sub(1) = 0, so offset = 0 * page_size = 0
        let p = Pagination::new(0, 10);
        assert_eq!(p.offset(), 0);
    }
}
