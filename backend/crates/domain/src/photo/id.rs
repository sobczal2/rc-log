use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PhotoId(Uuid);

impl PhotoId {
    pub fn new(uuid: Uuid) -> Self {
        Self(uuid)
    }

    pub fn as_uuid(&self) -> Uuid {
        self.0
    }
}

impl From<Uuid> for PhotoId {
    fn from(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

impl From<PhotoId> for Uuid {
    fn from(id: PhotoId) -> Uuid {
        id.0
    }
}
