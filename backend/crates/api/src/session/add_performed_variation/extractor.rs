use axum::{
    Json,
    extract::{FromRequest, FromRequestParts, Path, Request},
};
use rc_log_application::session::add_performed_variation::model::RatingDto;
use rc_log_application::shared::validator::ValidationError;
use serde::Deserialize;
use uuid::Uuid;

use crate::error::ApiError;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AddPerformedVariationBody {
    variation_id: Uuid,
    quality: RatingDto,
    comfort: RatingDto,
    repeatability: RatingDto,
    note: Option<String>,
}

pub struct AddPerformedVariationRequest {
    pub session_id: Uuid,
    pub variation_id: Uuid,
    pub quality: RatingDto,
    pub comfort: RatingDto,
    pub repeatability: RatingDto,
    pub note: Option<String>,
}

impl<S> FromRequest<S> for AddPerformedVariationRequest
where
    S: Send + Sync,
{
    type Rejection = ApiError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let mut parts = req.into_parts();

        let session_id = {
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
        let Json(body): Json<AddPerformedVariationBody> = Json::from_request(reconstructed, state)
            .await
            .map_err(|e| ApiError::Validation(vec![ValidationError::new("body", e.to_string())]))?;

        if body.variation_id.is_nil() {
            return Err(ApiError::Validation(vec![ValidationError::new(
                "variationId",
                "must not be nil",
            )]));
        }

        Ok(Self {
            session_id,
            variation_id: body.variation_id,
            quality: body.quality,
            comfort: body.comfort,
            repeatability: body.repeatability,
            note: body.note,
        })
    }
}
