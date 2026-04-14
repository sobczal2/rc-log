use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TrainingProgramPartId(Uuid);

impl TrainingProgramPartId {
    pub fn new(uuid: Uuid) -> Self {
        Self(uuid)
    }

    pub fn as_uuid(&self) -> Uuid {
        self.0
    }
}

impl From<Uuid> for TrainingProgramPartId {
    fn from(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

impl From<TrainingProgramPartId> for Uuid {
    fn from(id: TrainingProgramPartId) -> Uuid {
        id.0
    }
}
