use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ModelId(Uuid);

impl ModelId {
    pub fn new(uuid: Uuid) -> Self {
        Self(uuid)
    }

    pub fn as_uuid(&self) -> Uuid {
        self.0
    }
}

impl From<Uuid> for ModelId {
    fn from(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

impl From<ModelId> for Uuid {
    fn from(id: ModelId) -> Uuid {
        id.0
    }
}
