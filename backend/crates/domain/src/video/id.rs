use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VideoId(Uuid);

impl VideoId {
    pub fn new(uuid: Uuid) -> Self {
        Self(uuid)
    }

    pub fn as_uuid(&self) -> Uuid {
        self.0
    }
}

impl From<Uuid> for VideoId {
    fn from(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

impl From<VideoId> for Uuid {
    fn from(id: VideoId) -> Uuid {
        id.0
    }
}
