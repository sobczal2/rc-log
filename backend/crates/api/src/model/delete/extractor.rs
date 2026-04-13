use axum::{
    extract::{FromRequestParts, Path},
    http::request::Parts,
};
use rc_log_application::shared::validator::ValidationError;
use uuid::Uuid;

use crate::error::ApiError;

#[derive(Debug)]
pub struct DeleteRequest(pub Uuid);

impl<S> FromRequestParts<S> for DeleteRequest
where
    S: Send + Sync,
{
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let Path(id) = Path::<Uuid>::from_request_parts(parts, state)
            .await
            .map_err(|e| ApiError::Validation(vec![ValidationError::new("id", e.to_string())]))?;

        if id.is_nil() {
            return Err(ApiError::Validation(vec![ValidationError::new(
                "id",
                "must not be nil",
            )]));
        }

        Ok(Self(id))
    }
}
