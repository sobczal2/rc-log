use axum::{
    extract::{FromRequestParts, Path},
    http::request::Parts,
};
use rc_log_application::photo::resolve::model::ResolvePhotoInput;
use rc_log_application::shared::validator::{Validate, ValidationError};

use crate::error::ApiError;

#[derive(Debug)]
pub struct ResolvePhotoRequest(pub ResolvePhotoInput);

impl<S> FromRequestParts<S> for ResolvePhotoRequest
where
    S: Send + Sync,
{
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let Path(name) = Path::<String>::from_request_parts(parts, state)
            .await
            .map_err(|e| ApiError::Validation(vec![ValidationError::new("name", e.to_string())]))?;

        let input = ResolvePhotoInput { name };

        if let Err(errors) = input.validate() {
            return Err(ApiError::Validation(errors));
        }

        Ok(Self(input))
    }
}
