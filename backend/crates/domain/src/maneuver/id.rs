use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ManeuverId(Uuid);

impl ManeuverId {
    pub fn new(uuid: Uuid) -> Self {
        Self(uuid)
    }

    pub fn as_uuid(&self) -> Uuid {
        self.0
    }
}

impl From<Uuid> for ManeuverId {
    fn from(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

impl From<ManeuverId> for Uuid {
    fn from(id: ManeuverId) -> Uuid {
        id.0
    }
}
