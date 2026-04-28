pub mod id;
pub mod model_resolver;
pub mod name;
pub mod transaction;
pub mod r#type;

pub use r#type::Type;

use crate::photo::PhotoId;
use crate::model::id::ModelId;
use crate::model::name::Name;
use crate::user::id::UserId;

#[derive(Debug, Clone)]
pub struct Model {
    id: ModelId,
    owner_id: UserId,
    name: Name,
    r#type: Type,
    photo_asset_id: Option<PhotoId>,
}

impl Model {
    pub fn new(
        id: ModelId,
        owner_id: UserId,
        name: Name,
        r#type: Type,
        photo_asset_id: Option<PhotoId>,
    ) -> Self {
        Self { id, owner_id, name, r#type, photo_asset_id }
    }

    pub fn id(&self) -> ModelId {
        self.id
    }

    pub fn owner_id(&self) -> UserId {
        self.owner_id
    }

    pub fn name(&self) -> &Name {
        &self.name
    }

    pub fn r#type(&self) -> Type {
        self.r#type
    }

    pub fn photo_asset_id(&self) -> Option<&PhotoId> {
        self.photo_asset_id.as_ref()
    }
}
