use axum::{
    extract::{FromRequestParts, Path},
    http::request::Parts,
};
use rc_log_application::shared::validator::{Validate, ValidationError};
use rc_log_application::video::resolve::model::ResolveVideoInput;

use crate::error::ApiError;

#[derive(Debug)]
pub struct ResolveVideoRequest(pub ResolveVideoInput);

impl<S> FromRequestParts<S> for ResolveVideoRequest
where
    S: Send + Sync,
{
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let Path(id) = Path::<String>::from_request_parts(parts, state)
            .await
            .map_err(|e| ApiError::Validation(vec![ValidationError::new("id", e.to_string())]))?;

        let input = ResolveVideoInput { id };

        if let Err(errors) = input.validate() {
            return Err(ApiError::Validation(errors));
        }

        Ok(Self(input))
    }
}
