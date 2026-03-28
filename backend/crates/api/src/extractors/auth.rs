use axum::{
    extract::{FromRef, FromRequestParts},
    http::{header::AUTHORIZATION, request::Parts},
};
use uuid::Uuid;

use crate::error::ApiError;
use crate::jwt::verify_token;
use crate::state::AppState;

#[derive(Debug)]
#[allow(dead_code)]
pub struct AuthenticatedUser {
    pub id: Uuid,
    pub username: String,
}

impl<S> FromRequestParts<S> for AuthenticatedUser
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let app_state = AppState::from_ref(state);

        let auth_header = parts
            .headers
            .get(AUTHORIZATION)
            .and_then(|v| v.to_str().ok())
            .ok_or(ApiError::Unauthorized)?;

        let token = auth_header.strip_prefix("Bearer ").ok_or(ApiError::Unauthorized)?;

        let claims =
            verify_token(token, &app_state.jwt_secret).map_err(|_| ApiError::Unauthorized)?;

        Ok(AuthenticatedUser { id: claims.sub, username: claims.username })
    }
}
