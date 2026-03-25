use axum::{
    extract::{FromRequestParts, Query},
    http::{StatusCode, request::Parts},
    response::{IntoResponse, Response},
    Json,
};
use rc_log_application::shared::pagination::PaginationDto;
use serde::Deserialize;
use serde_json::json;

const DEFAULT_PAGE: u32 = 1;
const DEFAULT_PAGE_SIZE: u32 = 20;
const MAX_PAGE_SIZE: u32 = 100;

#[derive(Debug, Deserialize)]
pub struct PaginationQuery {
    #[serde(default = "default_page")]
    pub page: u32,
    #[serde(default = "default_page_size")]
    pub page_size: u32,
}

fn default_page() -> u32 {
    DEFAULT_PAGE
}

fn default_page_size() -> u32 {
    DEFAULT_PAGE_SIZE
}

impl PaginationQuery {
    fn validate(&self) -> Result<(), Response> {
        if self.page == 0 {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(json!({ "error": "page must be >= 1" })),
            )
                .into_response());
        }
        if self.page_size == 0 || self.page_size > MAX_PAGE_SIZE {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "error": format!("page_size must be between 1 and {}", MAX_PAGE_SIZE)
                })),
            )
                .into_response());
        }
        Ok(())
    }

    pub fn into_dto(self) -> PaginationDto {
        PaginationDto { page: self.page, page_size: self.page_size }
    }
}

impl<S> FromRequestParts<S> for PaginationQuery
where
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let Query(query) = Query::<PaginationQuery>::from_request_parts(parts, state)
            .await
            .map_err(|e| {
                (
                    StatusCode::BAD_REQUEST,
                    Json(json!({ "error": e.body_text() })),
                )
                    .into_response()
            })?;

        query.validate()?;
        Ok(query)
    }
}
