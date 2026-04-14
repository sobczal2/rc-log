use axum::{
    Json,
    extract::{FromRequest, FromRequestParts, Path, Request},
};
use rc_log_application::shared::validator::ValidationError;
use serde::Deserialize;
use uuid::Uuid;

use crate::error::ApiError;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct UpdateSessionBody {
    date: String,
    model_id: Option<Uuid>,
    note: Option<String>,
}

pub struct UpdateSessionRequest {
    pub id: Uuid,
    pub date: String,
    pub model_id: Option<Uuid>,
    pub note: Option<String>,
}

impl<S> FromRequest<S> for UpdateSessionRequest
where
    S: Send + Sync,
{
    type Rejection = ApiError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let mut parts = req.into_parts();

        let id = {
            let Path(id) =
                Path::<Uuid>::from_request_parts(&mut parts.0, state).await.map_err(|e| {
                    ApiError::Validation(vec![ValidationError::new("id", e.to_string())])
                })?;

            if id.is_nil() {
                return Err(ApiError::Validation(vec![ValidationError::new(
                    "id",
                    "must not be nil",
                )]));
            }

            id
        };

        let reconstructed = Request::from_parts(parts.0, parts.1);
        let Json(body): Json<UpdateSessionBody> = Json::from_request(reconstructed, state)
            .await
            .map_err(|e| ApiError::Validation(vec![ValidationError::new("body", e.to_string())]))?;

        if body.date.trim().is_empty() {
            return Err(ApiError::Validation(vec![ValidationError::new(
                "date",
                "must not be empty",
            )]));
        }

        Ok(Self { id, date: body.date, model_id: body.model_id, note: body.note })
    }
}
