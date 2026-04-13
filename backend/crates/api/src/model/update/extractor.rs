use axum::{
    Json,
    extract::{FromRequest, FromRequestParts, Path, Request},
};
use rc_log_application::shared::validator::ValidationError;
use rc_log_application::shared::vehicle_type::VehicleTypeDto;
use serde::Deserialize;
use uuid::Uuid;

use crate::error::ApiError;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct UpdateModelBody {
    name: String,
    vehicle_type: String,
}

pub struct UpdateRequest {
    pub id: Uuid,
    pub name: String,
    pub vehicle_type: VehicleTypeDto,
}

impl<S> FromRequest<S> for UpdateRequest
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
        let Json(body): Json<UpdateModelBody> = Json::from_request(reconstructed, state)
            .await
            .map_err(|e| ApiError::Validation(vec![ValidationError::new("body", e.to_string())]))?;

        if body.name.is_empty() {
            return Err(ApiError::Validation(vec![ValidationError::new(
                "name",
                "must not be empty",
            )]));
        }

        let vehicle_type = match body.vehicle_type.as_str() {
            "Helicopter" => VehicleTypeDto::Helicopter,
            "Plane" => VehicleTypeDto::Plane,
            "Drone" => VehicleTypeDto::Drone,
            _ => {
                return Err(ApiError::Validation(vec![ValidationError::new(
                    "vehicleType",
                    "must be one of: Helicopter, Plane, Drone",
                )]));
            }
        };

        Ok(Self { id, name: body.name, vehicle_type })
    }
}
