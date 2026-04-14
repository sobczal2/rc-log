use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PerformedVariationId(Uuid);

impl PerformedVariationId {
    pub fn new(uuid: Uuid) -> Self {
        Self(uuid)
    }

    pub fn as_uuid(&self) -> Uuid {
        self.0
    }
}

impl From<Uuid> for PerformedVariationId {
    fn from(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

impl From<PerformedVariationId> for Uuid {
    fn from(id: PerformedVariationId) -> Uuid {
        id.0
    }
}
