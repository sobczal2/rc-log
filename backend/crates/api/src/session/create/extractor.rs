use axum::{
    Json,
    extract::{FromRequest, Request},
};
use serde::Deserialize;
use uuid::Uuid;

use crate::error::ApiError;
use rc_log_application::shared::validator::ValidationError;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CreateSessionBody {
    date: String,
    model_id: Option<Uuid>,
    note: Option<String>,
}

pub struct CreateSessionRequest {
    pub date: String,
    pub model_id: Option<Uuid>,
    pub note: Option<String>,
}

impl<S> FromRequest<S> for CreateSessionRequest
where
    S: Send + Sync,
{
    type Rejection = ApiError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(body): Json<CreateSessionBody> = Json::from_request(req, state)
            .await
            .map_err(|e| ApiError::Validation(vec![ValidationError::new("body", e.to_string())]))?;

        if body.date.trim().is_empty() {
            return Err(ApiError::Validation(vec![ValidationError::new(
                "date",
                "must not be empty",
            )]));
        }

        Ok(Self { date: body.date, model_id: body.model_id, note: body.note })
    }
}
