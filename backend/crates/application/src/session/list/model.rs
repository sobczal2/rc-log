use rc_log_domain::maneuver::id::ManeuverId;
use rc_log_domain::session::transaction::{SessionFilter, SessionSort, SessionSortField};
use rc_log_domain::shared::sort::SortDirection;
use crate::shared::sort::SortDirectionDto;
use serde::Serialize;
use specta::Type;
use uuid::Uuid;

use crate::model::shared::TypeDto;
pub use crate::session::shared::rating::RatingDto;
use crate::shared::pagination::PaginationDto;
use crate::shared::validator::{Validate, ValidationError};

#[derive(Debug, Clone)]
pub struct ListSessionsInput {
    pub owner_id: Uuid,
    pub pagination: PaginationDto,
    pub filter: SessionFilterDto,
    pub sort: SessionSortDto,
}

#[derive(Debug, Clone)]
pub struct SessionFilterDto {
    pub model_ids: Vec<Uuid>,
    pub maneuver_ids: Vec<Uuid>,
    pub search_query: Option<String>,
}

impl Validate for SessionFilterDto {
    fn validate(&self) -> Result<(), Vec<ValidationError>> {
        let mut errors = Vec::new();

        for (idx, id) in self.model_ids.iter().enumerate() {
            if id.is_nil() {
                errors.push(ValidationError::new(format!("modelIds[{idx}]"), "must not be nil"));
            }
        }

        for (idx, id) in self.maneuver_ids.iter().enumerate() {
            if id.is_nil() {
                errors.push(ValidationError::new(format!("maneuverIds[{idx}]"), "must not be nil"));
            }
        }

        if let Some(search_query) = &self.search_query && search_query.len() > 200 {
            errors.push(ValidationError::new("searchQuery", "must not exceed 200 characters"));
        }

        if errors.is_empty() { Ok(()) } else { Err(errors) }
    }
}

impl From<SessionFilterDto> for SessionFilter {
    fn from(dto: SessionFilterDto) -> Self {
        Self {
            model_ids: dto.model_ids.into_iter().map(Into::into).collect(),
            maneuver_ids: dto.maneuver_ids.into_iter().map(ManeuverId::new).collect(),
            search_query: dto.search_query,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum SessionSortFieldDto {
    Date,
}

#[derive(Debug, Clone)]
pub struct SessionSortDto {
    pub field: Option<SessionSortFieldDto>,
    pub direction: Option<SortDirectionDto>,
}

impl Validate for SessionSortDto {
    fn validate(&self) -> Result<(), Vec<ValidationError>> {
        // validation is handled by the extractor; enum-backed DTOs are valid by construction
        Ok(())
    }
}

impl From<SessionSortDto> for SessionSort {
    fn from(dto: SessionSortDto) -> Self {
        let field = match dto.field.unwrap_or(SessionSortFieldDto::Date) {
            SessionSortFieldDto::Date => SessionSortField::Date,
        };

        let direction = match dto.direction.unwrap_or(SortDirectionDto::default()) {
            SortDirectionDto::Asc => SortDirection::Asc,
            SortDirectionDto::Desc => SortDirection::Desc,
        };

        Self { field, direction }
    }
}

impl Validate for ListSessionsInput {
    fn validate(&self) -> Result<(), Vec<ValidationError>> {
        let mut errors = Vec::new();

        if let Err(mut errs) = self.pagination.validate() {
            errors.append(&mut errs);
        }

        if let Err(mut errs) = self.filter.validate() {
            errors.append(&mut errs);
        }

        if let Err(mut errs) = self.sort.validate() {
            errors.append(&mut errs);
        }

        if errors.is_empty() { Ok(()) } else { Err(errors) }
    }
}

#[derive(Debug, Clone, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct PerformedVariationDto {
    pub performed_variation_id: Uuid,
    pub variation_id: Uuid,
    pub maneuver_name: Option<String>,
    pub variation_name: Option<String>,
    pub quality: RatingDto,
    pub comfort: RatingDto,
    pub repeatability: RatingDto,
}

#[derive(Debug, Clone, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct SessionDto {
    pub id: Uuid,
    pub user_id: Uuid,
    pub date: String,
    pub model_id: Option<Uuid>,
    pub model_name: Option<String>,
    pub model_type: Option<TypeDto>,
    pub model_photo_asset_id: Option<String>,
    pub performed_variations: Vec<PerformedVariationDto>,
}
