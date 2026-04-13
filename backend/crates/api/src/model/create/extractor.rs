use axum::{
    Json,
    extract::{FromRequest, Request},
};
use rc_log_application::shared::validator::ValidationError;
use rc_log_application::shared::vehicle_type::VehicleTypeDto;
use serde::Deserialize;

use crate::error::ApiError;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CreateModelBody {
    name: String,
    vehicle_type: String,
}

pub struct CreateRequest {
    pub name: String,
    pub vehicle_type: VehicleTypeDto,
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

        Ok(Self { name: body.name, vehicle_type })
    }
}
