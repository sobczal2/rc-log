use axum::{
    extract::{FromRequestParts, Path},
    http::request::Parts,
};
use rc_log_application::maneuver::get_by_id::model::GetManeuverByIdInput;
use rc_log_application::shared::validator::{Validate, ValidationError};
use uuid::Uuid;

use crate::error::ApiError;

#[derive(Debug)]
pub struct GetByIdRequest(pub GetManeuverByIdInput);

impl<S> FromRequestParts<S> for GetByIdRequest
where
    S: Send + Sync,
{
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let Path(id) = Path::<Uuid>::from_request_parts(parts, state)
            .await
            .map_err(|e| ApiError::Validation(vec![ValidationError::new("id", e.to_string())]))?;

        let input = GetManeuverByIdInput { id };

        if let Err(errors) = input.validate() {
            return Err(ApiError::Validation(errors));
        }

        Ok(Self(input))
    }
}
