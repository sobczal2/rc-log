use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VariationId(Uuid);

impl VariationId {
    pub fn new(uuid: Uuid) -> Self {
        Self(uuid)
    }

    pub fn as_uuid(&self) -> Uuid {
        self.0
    }
}

impl From<Uuid> for VariationId {
    fn from(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

impl From<VariationId> for Uuid {
    fn from(id: VariationId) -> Uuid {
        id.0
    }
}
