use rc_log_domain::model::Model;
use rc_log_domain::shared::vehicle_type::VehicleType;
use serde::Serialize;
use uuid::Uuid;

use crate::shared::validator::{Validate, ValidationError};
use crate::shared::vehicle_type::VehicleTypeDto;

#[derive(Debug, Clone)]
pub struct GetModelByIdInput {
    pub id: Uuid,
    pub owner_id: Uuid,
}

impl Validate for GetModelByIdInput {
    fn validate(&self) -> Result<(), Vec<ValidationError>> {
        if self.id.is_nil() {
            return Err(vec![ValidationError::new("id", "must not be a nil UUID")]);
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ModelDto {
    pub id: Uuid,
    pub owner_id: Uuid,
    pub name: String,
    pub vehicle_type: VehicleTypeDto,
    pub photo_asset_name: Option<String>,
}

impl From<Model> for ModelDto {
    fn from(m: Model) -> Self {
        let vehicle_type = match m.vehicle_type() {
            VehicleType::Helicopter => VehicleTypeDto::Helicopter,
            VehicleType::Plane => VehicleTypeDto::Plane,
            VehicleType::Drone => VehicleTypeDto::Drone,
        };
        Self {
            id: Uuid::from(m.id()),
            owner_id: Uuid::from(m.owner_id()),
            name: m.name().as_str().to_string(),
            vehicle_type,
            photo_asset_name: m.photo_asset_name().map(|n| n.as_str().to_string()),
        }
    }
}
