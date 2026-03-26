use axum::{
    extract::{FromRequestParts, Query},
    http::request::Parts,
};
use rc_log_application::maneuver::list::model::{
    ListManeuversInput, ManeuverFilterDto, ManeuverSortDto,
};
use rc_log_application::shared::pagination::PaginationDto;
use rc_log_application::shared::validator::Validate;
use serde::Deserialize;

use crate::error::ApiError;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct RawListQuery {
    page: u32,
    page_size: u32,
    #[serde(default)]
    tags: String,
    #[serde(default)]
    vehicle_type: String,
    #[serde(default)]
    difficulty: String,
    #[serde(default)]
    search_query: String,
    #[serde(default)]
    sort_field: String,
    #[serde(default)]
    sort_direction: String,
}

#[derive(Debug)]
pub struct ListRequest(pub ListManeuversInput);

impl<S> FromRequestParts<S> for ListRequest
where
    S: Send + Sync,
{
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let Query(raw) = Query::<RawListQuery>::from_request_parts(parts, state)
            .await
            .map_err(|e| ApiError::Validation(vec![rc_log_application::shared::validator::ValidationError::new("query", e.to_string())]))?;

        let tags = raw.tags
            .split(',')
            .map(|t| t.trim().to_string())
            .filter(|t| !t.is_empty())
            .collect();

        use rc_log_application::shared::vehicle_type::VehicleTypeDto;
        let vehicle_type = match raw.vehicle_type.as_str() {
            "Helicopter" => Some(VehicleTypeDto::Helicopter),
            "Plane" => Some(VehicleTypeDto::Plane),
            "Drone" => Some(VehicleTypeDto::Drone),
            "" => None,
            _ => return Err(ApiError::Validation(vec![rc_log_application::shared::validator::ValidationError::new("vehicle_type", "invalid vehicle type")])),
        };

        use rc_log_application::shared::difficulty::DifficultyDto;
        let difficulty = match raw.difficulty.as_str() {
            "Level1" => Some(DifficultyDto::Level1),
            "Level2" => Some(DifficultyDto::Level2),
            "Level3" => Some(DifficultyDto::Level3),
            "Level4" => Some(DifficultyDto::Level4),
            "Level5" => Some(DifficultyDto::Level5),
            "Level6" => Some(DifficultyDto::Level6),
            "Level7" => Some(DifficultyDto::Level7),
            "" => None,
            _ => return Err(ApiError::Validation(vec![rc_log_application::shared::validator::ValidationError::new("difficulty", "invalid difficulty")])),
        };

        let search_query = if raw.search_query.is_empty() { None } else { Some(raw.search_query) };

        let input = ListManeuversInput {
            pagination: PaginationDto { page: raw.page, page_size: raw.page_size },
            filter: ManeuverFilterDto {
                tags,
                vehicle_type,
                difficulty,
                search_query,
            },
            sort: ManeuverSortDto {
                field: raw.sort_field,
                direction: raw.sort_direction,
            },
        };

        if let Err(errors) = input.validate() {
            return Err(ApiError::Validation(errors));
        }

        Ok(Self(input))
    }
}
