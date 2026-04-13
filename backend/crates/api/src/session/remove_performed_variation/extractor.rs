use axum::{
    extract::{FromRequestParts, Path},
    http::request::Parts,
};
use rc_log_application::shared::validator::ValidationError;
use uuid::Uuid;

use crate::error::ApiError;

pub struct RemovePerformedVariationRequest {
    pub session_id: Uuid,
    pub variation_id: Uuid,
}

impl<S> FromRequestParts<S> for RemovePerformedVariationRequest
where
    S: Send + Sync,
{
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let Path((session_id, variation_id)) =
            Path::<(Uuid, Uuid)>::from_request_parts(parts, state).await.map_err(|e| {
                ApiError::Validation(vec![ValidationError::new("path", e.to_string())])
            })?;

        if session_id.is_nil() {
            return Err(ApiError::Validation(vec![ValidationError::new(
                "id",
                "must not be nil",
            )]));
        }

        if variation_id.is_nil() {
            return Err(ApiError::Validation(vec![ValidationError::new(
                "variationId",
                "must not be nil",
            )]));
        }

        Ok(Self { session_id, variation_id })
    }
}
