use rc_log_domain::maneuver::id::ManeuverId;
use rc_log_domain::session::Session;
use rc_log_domain::session::rating::{Comfort, Quality, Repeatability};
use rc_log_domain::session::transaction::{
    SessionFilter, SessionSort, SessionSortField, SortDirection,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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
                errors.push(ValidationError::new(
                    format!("maneuverIds[{idx}]"),
                    "must not be nil",
                ));
            }
        }

        if let Some(search_query) = &self.search_query {
            if search_query.len() > 200 {
                errors.push(ValidationError::new(
                    "searchQuery",
                    "must not exceed 200 characters",
                ));
            }
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

#[derive(Debug, Clone)]
pub struct SessionSortDto {
    pub field: String,
    pub direction: String,
}

impl Validate for SessionSortDto {
    fn validate(&self) -> Result<(), Vec<ValidationError>> {
        let mut errors = Vec::new();
        let valid_fields = ["", "date"];
        if !valid_fields.contains(&self.field.as_str()) {
            errors.push(ValidationError::new("sort.field", "must be 'date'"));
        }

        let valid_directions = ["", "asc", "desc"];
        if !valid_directions.contains(&self.direction.as_str()) {
            errors.push(ValidationError::new("sort.direction", "must be 'asc' or 'desc'"));
        }

        if errors.is_empty() { Ok(()) } else { Err(errors) }
    }
}

impl From<SessionSortDto> for SessionSort {
    fn from(dto: SessionSortDto) -> Self {
        let field = match dto.field.to_lowercase().as_str() {
            "date" => SessionSortField::Date,
            _ => SessionSortField::Date,
        };

        let direction = match dto.direction.to_lowercase().as_str() {
            "asc" => SortDirection::Asc,
            _ => SortDirection::Desc,
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum QualityDto {
    One,
    Two,
    Three,
    Four,
    Five,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum ComfortDto {
    One,
    Two,
    Three,
    Four,
    Five,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum RepeatabilityDto {
    One,
    Two,
    Three,
    Four,
    Five,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PerformedVariationDto {
    pub variation_id: Uuid,
    pub quality: QualityDto,
    pub comfort: ComfortDto,
    pub repeatability: RepeatabilityDto,
    pub note: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionDto {
    pub id: Uuid,
    pub user_id: Uuid,
    pub date: String,
    pub model_id: Option<Uuid>,
    pub note: Option<String>,
    pub performed_variations: Vec<PerformedVariationDto>,
}

impl From<Session> for SessionDto {
    fn from(session: Session) -> Self {
        let performed_variations = session
            .performed_variations()
            .iter()
            .map(|performed| {
                let rating = performed.rating();
                let quality = match rating.quality() {
                    Quality::One => QualityDto::One,
                    Quality::Two => QualityDto::Two,
                    Quality::Three => QualityDto::Three,
                    Quality::Four => QualityDto::Four,
                    Quality::Five => QualityDto::Five,
                };
                let comfort = match rating.comfort() {
                    Comfort::One => ComfortDto::One,
                    Comfort::Two => ComfortDto::Two,
                    Comfort::Three => ComfortDto::Three,
                    Comfort::Four => ComfortDto::Four,
                    Comfort::Five => ComfortDto::Five,
                };
                let repeatability = match rating.repeatability() {
                    Repeatability::One => RepeatabilityDto::One,
                    Repeatability::Two => RepeatabilityDto::Two,
                    Repeatability::Three => RepeatabilityDto::Three,
                    Repeatability::Four => RepeatabilityDto::Four,
                    Repeatability::Five => RepeatabilityDto::Five,
                };

                PerformedVariationDto {
                    variation_id: Uuid::from(performed.variation_id()),
                    quality,
                    comfort,
                    repeatability,
                    note: performed.note().map(|n| n.as_str().to_string()),
                }
            })
            .collect();

        Self {
            id: Uuid::from(session.id()),
            user_id: Uuid::from(session.user_id()),
            date: session.date().as_naive_date().format("%Y-%m-%d").to_string(),
            model_id: session.model_id().map(Uuid::from),
            note: session.note().map(|n| n.as_str().to_string()),
            performed_variations,
        }
    }
}
