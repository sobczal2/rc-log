use axum::{
    Json,
    extract::{FromRequest, FromRequestParts, Path, Request},
};
use rc_log_application::session::update_performed_variation::model::{
    ComfortDto, QualityDto, RepeatabilityDto,
};
use rc_log_application::shared::validator::ValidationError;
use serde::Deserialize;
use uuid::Uuid;

use crate::error::ApiError;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct UpdatePerformedVariationBody {
    quality: QualityDto,
    comfort: ComfortDto,
    repeatability: RepeatabilityDto,
    note: Option<String>,
}

pub struct UpdatePerformedVariationRequest {
    pub session_id: Uuid,
    pub performed_variation_id: Uuid,
    pub quality: QualityDto,
    pub comfort: ComfortDto,
    pub repeatability: RepeatabilityDto,
    pub note: Option<String>,
}

impl<S> FromRequest<S> for UpdatePerformedVariationRequest
where
    S: Send + Sync,
{
    type Rejection = ApiError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let mut parts = req.into_parts();

        let (session_id, performed_variation_id) = {
            let Path((session_id, performed_variation_id)) =
                Path::<(Uuid, Uuid)>::from_request_parts(&mut parts.0, state)
                    .await
                    .map_err(|e| {
                        ApiError::Validation(vec![ValidationError::new("path", e.to_string())])
                    })?;

            if session_id.is_nil() {
                return Err(ApiError::Validation(vec![ValidationError::new(
                    "id",
                    "must not be nil",
                )]));
            }

            if performed_variation_id.is_nil() {
                return Err(ApiError::Validation(vec![ValidationError::new(
                    "performedVariationId",
                    "must not be nil",
                )]));
            }

            (session_id, performed_variation_id)
        };

        let reconstructed = Request::from_parts(parts.0, parts.1);
        let Json(body): Json<UpdatePerformedVariationBody> =
            Json::from_request(reconstructed, state).await.map_err(|e| {
                ApiError::Validation(vec![ValidationError::new("body", e.to_string())])
            })?;

        Ok(Self {
            session_id,
            performed_variation_id,
            quality: body.quality,
            comfort: body.comfort,
            repeatability: body.repeatability,
            note: body.note,
        })
    }
}
