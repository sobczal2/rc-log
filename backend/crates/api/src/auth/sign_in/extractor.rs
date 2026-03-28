use axum::{Json, extract::FromRequest};
use serde::Deserialize;

use rc_log_application::shared::validator::ValidationError;
use rc_log_application::user::sign_in::model::SignInInput;

use crate::error::ApiError;

#[derive(Debug, Deserialize)]
struct SignInBody {
    username: String,
    password: String,
}

pub struct SignInRequest(pub SignInInput);

impl<S> FromRequest<S> for SignInRequest
where
    S: Send + Sync,
{
    type Rejection = ApiError;

    async fn from_request(req: axum::extract::Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(body): Json<SignInBody> = Json::from_request(req, state)
            .await
            .map_err(|e| ApiError::Validation(vec![ValidationError::new("body", e.to_string())]))?;

        if body.username.is_empty() {
            return Err(ApiError::Validation(vec![ValidationError::new(
                "username",
                "must not be empty",
            )]));
        }
        if body.password.is_empty() {
            return Err(ApiError::Validation(vec![ValidationError::new(
                "password",
                "must not be empty",
            )]));
        }

        Ok(Self(SignInInput { username: body.username, password: body.password }))
    }
}
