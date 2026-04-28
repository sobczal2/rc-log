use axum::{
    extract::{FromRequestParts, Query},
    http::request::Parts,
};
use rc_log_application::session::list::model::{SessionFilterDto, SessionSortDto, SessionSortFieldDto};
use rc_log_application::shared::sort::SortDirectionDto;
use rc_log_application::shared::pagination::PaginationDto;
use rc_log_application::shared::validator::{Validate, ValidationError};
use serde::Deserialize;
use uuid::Uuid;

use crate::error::ApiError;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RawListQuery {
    #[serde(default = "default_page")]
    page: u32,
    #[serde(default = "default_page_size")]
    page_size: u32,
    #[serde(default)]
    model_ids: String,
    #[serde(default)]
    maneuver_ids: String,
    #[serde(default)]
    search_query: String,
    #[serde(default)]
    sort_field: String,
    #[serde(default)]
    sort_direction: String,
}

fn default_page() -> u32 {
    1
}

fn default_page_size() -> u32 {
    20
}

#[derive(Debug)]
pub struct ListRequest {
    pub pagination: PaginationDto,
    pub filter: SessionFilterDto,
    pub sort: SessionSortDto,
}

impl<S> FromRequestParts<S> for ListRequest
where
    S: Send + Sync,
{
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let Query(raw) =
            Query::<RawListQuery>::from_request_parts(parts, state).await.map_err(|e| {
                ApiError::Validation(vec![ValidationError::new("query", e.to_string())])
            })?;

        let parse_uuid_csv = |field: &str, value: &str| -> Result<Vec<Uuid>, ApiError> {
            let mut ids = Vec::new();
            for token in value.split(',').map(|x| x.trim()).filter(|x| !x.is_empty()) {
                let id = Uuid::parse_str(token).map_err(|_| {
                    ApiError::Validation(vec![ValidationError::new(
                        field,
                        format!("invalid UUID: {token}"),
                    )])
                })?;
                ids.push(id);
            }
            Ok(ids)
        };

        let model_ids = parse_uuid_csv("modelIds", &raw.model_ids)?;
        let maneuver_ids = parse_uuid_csv("maneuverIds", &raw.maneuver_ids)?;

        let pagination = PaginationDto { page: raw.page, page_size: raw.page_size };
        let filter = SessionFilterDto {
            model_ids,
            maneuver_ids,
            search_query: if raw.search_query.trim().is_empty() {
                None
            } else {
                Some(raw.search_query)
            },
        };
        let sort_field = match raw.sort_field.as_str() {
            "" => None,
            "date" => Some(SessionSortFieldDto::Date),
            _ => return Err(ApiError::Validation(vec![ValidationError::new("sort.field", "must be 'date'")])),
        };

        let sort_direction = match raw.sort_direction.as_str() {
            "" => None,
            "asc" => Some(SortDirectionDto::Asc),
            "desc" => Some(SortDirectionDto::Desc),
            _ => return Err(ApiError::Validation(vec![ValidationError::new("sort.direction", "must be 'asc' or 'desc'")])),
        };

        let sort = SessionSortDto { field: sort_field, direction: sort_direction };

        let mut errors = Vec::new();
        if let Err(mut errs) = pagination.validate() {
            errors.append(&mut errs);
        }
        if let Err(mut errs) = filter.validate() {
            errors.append(&mut errs);
        }
        if let Err(mut errs) = sort.validate() {
            errors.append(&mut errs);
        }

        if !errors.is_empty() {
            return Err(ApiError::Validation(errors));
        }

        Ok(Self { pagination, filter, sort })
    }
}
