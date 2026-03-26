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
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
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
