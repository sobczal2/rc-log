use axum::{
    extract::{FromRequestParts, Query},
    http::request::Parts,
};
use rc_log_application::shared::pagination::PaginationDto;
use rc_log_application::shared::validator::{Validate, ValidationError};
use serde::Deserialize;

use crate::error::ApiError;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RawListQuery {
    #[serde(default = "default_page")]
    page: u32,
    #[serde(default = "default_page_size")]
    page_size: u32,
}

fn default_page() -> u32 {
    1
}

fn default_page_size() -> u32 {
    20
}

#[derive(Debug)]
pub struct ListRequest(pub PaginationDto);

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

        let pagination = PaginationDto { page: raw.page, page_size: raw.page_size };

        if let Err(errors) = pagination.validate() {
            return Err(ApiError::Validation(errors));
        }

        Ok(Self(pagination))
    }
}
