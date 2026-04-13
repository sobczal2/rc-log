use axum::Json;
use axum::extract::{FromRequest, Request};
use serde::Deserialize;

use crate::error::ApiError;
use rc_log_application::shared::validator::ValidationError;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct UpdateUserBody {
    new_username: String,
}

pub struct UpdateUserRequest {
    pub new_username: String,
}

impl<S> FromRequest<S> for UpdateUserRequest
where
    S: Send + Sync,
{
    type Rejection = ApiError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(body) = Json::<UpdateUserBody>::from_request(req, state).await.map_err(|e| {
            ApiError::Validation(vec![ValidationError::new("body", e.to_string())])
        })?;

        if body.new_username.trim().is_empty() {
            return Err(ApiError::Validation(vec![ValidationError::new(
                "newUsername",
                "must not be empty",
            )]));
        }

        Ok(Self { new_username: body.new_username })
    }
}
