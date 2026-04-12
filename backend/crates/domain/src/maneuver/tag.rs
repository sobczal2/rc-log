use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TagId(Uuid);

impl TagId {
    pub fn new(uuid: Uuid) -> Self {
        Self(uuid)
    }

    pub fn as_uuid(&self) -> Uuid {
        self.0
    }
}

impl From<Uuid> for TagId {
    fn from(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

impl From<TagId> for Uuid {
    fn from(id: TagId) -> Uuid {
        id.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Tag {
    id: TagId,
    name: String,
}

impl Tag {
    pub fn new(id: TagId, name: String) -> Self {
        Self { id, name }
    }

    pub fn id(&self) -> TagId {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}
