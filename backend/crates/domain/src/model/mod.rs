pub mod id;
pub mod name;
pub mod transaction;

use crate::asset::name::AssetName;
use crate::model::id::ModelId;
use crate::model::name::ModelName;
use crate::shared::vehicle_type::VehicleType;
use crate::user::id::UserId;

#[derive(Debug, Clone)]
pub struct Model {
    id: ModelId,
    owner_id: UserId,
    name: ModelName,
    vehicle_type: VehicleType,
    photo_asset_name: Option<AssetName>,
}

impl Model {
    pub fn new(
        id: ModelId,
        owner_id: UserId,
        name: ModelName,
        vehicle_type: VehicleType,
        photo_asset_name: Option<AssetName>,
    ) -> Self {
        Self {
            id,
            owner_id,
            name,
            vehicle_type,
            photo_asset_name,
        }
    }

    pub fn id(&self) -> ModelId {
        self.id
    }

    pub fn owner_id(&self) -> UserId {
        self.owner_id
    }

    pub fn name(&self) -> &ModelName {
        &self.name
    }

    pub fn vehicle_type(&self) -> VehicleType {
        self.vehicle_type
    }

    pub fn photo_asset_name(&self) -> Option<&AssetName> {
        self.photo_asset_name.as_ref()
    }
}
