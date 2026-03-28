use crate::shared::validator::{Validate, ValidationError};
use rc_log_domain::shared::pagination::Pagination;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginationDto {
    pub page: u32,
    pub page_size: u32,
}

impl Validate for PaginationDto {
    fn validate(&self) -> Result<(), Vec<ValidationError>> {
        let mut errors = Vec::new();
        if self.page < 1 {
            errors.push(ValidationError::new("page", "must be greater than or equal to 1"));
        }
        if self.page_size < 1 || self.page_size > 100 {
            errors.push(ValidationError::new("page_size", "must be between 1 and 100"));
        }
        if errors.is_empty() { Ok(()) } else { Err(errors) }
    }
}

impl From<PaginationDto> for Pagination {
    fn from(dto: PaginationDto) -> Self {
        Self::new(dto.page, dto.page_size)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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

#[cfg(test)]
mod tests {
    use super::{PaginatedResult, PaginationDto};
    use crate::shared::validator::Validate;

    // --- PaginationDto::validate ---

    #[test]
    fn valid_pagination_passes() {
        let p = PaginationDto { page: 1, page_size: 20 };
        assert!(p.validate().is_ok());
    }

    #[test]
    fn page_zero_fails() {
        let errs = PaginationDto { page: 0, page_size: 20 }.validate().unwrap_err();
        assert!(errs.iter().any(|e| e.field == "page"));
    }

    #[test]
    fn page_size_zero_fails() {
        let errs = PaginationDto { page: 1, page_size: 0 }.validate().unwrap_err();
        assert!(errs.iter().any(|e| e.field == "page_size"));
    }

    #[test]
    fn page_size_100_passes() {
        assert!(PaginationDto { page: 1, page_size: 100 }.validate().is_ok());
    }

    #[test]
    fn page_size_101_fails() {
        let errs = PaginationDto { page: 1, page_size: 101 }.validate().unwrap_err();
        assert!(errs.iter().any(|e| e.field == "page_size"));
    }

    #[test]
    fn both_invalid_reports_two_errors() {
        let errs = PaginationDto { page: 0, page_size: 0 }.validate().unwrap_err();
        assert_eq!(errs.len(), 2);
    }

    // --- PaginatedResult::total_pages ---

    #[test]
    fn exact_multiple_total_pages() {
        let r = PaginatedResult::<i32>::new(vec![], 100, 1, 10);
        assert_eq!(r.total_pages(), 10);
    }

    #[test]
    fn remainder_rounds_up() {
        let r = PaginatedResult::<i32>::new(vec![], 101, 1, 10);
        assert_eq!(r.total_pages(), 11);
    }

    #[test]
    fn zero_total_gives_zero_pages() {
        let r = PaginatedResult::<i32>::new(vec![], 0, 1, 10);
        assert_eq!(r.total_pages(), 0);
    }

    #[test]
    fn total_less_than_page_size_gives_one_page() {
        let r = PaginatedResult::<i32>::new(vec![], 5, 1, 20);
        assert_eq!(r.total_pages(), 1);
    }
}
