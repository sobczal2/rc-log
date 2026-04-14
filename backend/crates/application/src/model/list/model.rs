use rc_log_domain::model::Model;
use rc_log_domain::model::Type;
use serde::Serialize;
use specta::Type as SpectaType;
use uuid::Uuid;

use crate::shared::TypeDto;
use crate::shared::pagination::PaginationDto;

#[derive(Debug, Clone)]
pub struct ListModelsInput {
    pub owner_id: Uuid,
    pub pagination: PaginationDto,
}

#[derive(Debug, Clone, Serialize, SpectaType)]
#[serde(rename_all = "camelCase")]
pub struct ModelDto {
    pub id: Uuid,
    pub owner_id: Uuid,
    pub name: String,
    pub r#type: TypeDto,
    pub photo_asset_id: Option<String>,
}

impl From<Model> for ModelDto {
    fn from(m: Model) -> Self {
        let r#type = match m.r#type() {
            Type::Helicopter => TypeDto::Helicopter,
            Type::Plane => TypeDto::Plane,
            Type::Drone => TypeDto::Drone,
        };
        Self {
            id: Uuid::from(m.id()),
            owner_id: Uuid::from(m.owner_id()),
            name: m.name().as_str().to_string(),
            r#type,
            photo_asset_id: m.photo_asset_id().map(|id| id.as_uuid().to_string()),
        }
    }
}
