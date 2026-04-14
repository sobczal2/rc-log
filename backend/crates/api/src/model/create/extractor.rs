use axum::{
    Json,
    extract::{FromRequest, Request},
};
use rc_log_application::shared::TypeDto;
use rc_log_application::shared::validator::ValidationError;
use serde::Deserialize;

use crate::error::ApiError;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CreateModelBody {
    name: String,
    r#type: String,
}

pub struct CreateRequest {
    pub name: String,
    pub r#type: TypeDto,
}

impl<S> FromRequest<S> for CreateRequest
where
    S: Send + Sync,
{
    type Rejection = ApiError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(body): Json<CreateModelBody> = Json::from_request(req, state)
            .await
            .map_err(|e| ApiError::Validation(vec![ValidationError::new("body", e.to_string())]))?;

        if body.name.is_empty() {
            return Err(ApiError::Validation(vec![ValidationError::new(
                "name",
                "must not be empty",
            )]));
        }

        let r#type = match body.r#type.as_str() {
            "Helicopter" => TypeDto::Helicopter,
            "Plane" => TypeDto::Plane,
            "Drone" => TypeDto::Drone,
            _ => {
                return Err(ApiError::Validation(vec![ValidationError::new(
                    "type",
                    "must be one of: Helicopter, Plane, Drone",
                )]));
            }
        };

        Ok(Self { name: body.name, r#type })
    }
}
