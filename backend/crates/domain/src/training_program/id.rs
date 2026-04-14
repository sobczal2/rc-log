use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TrainingProgramId(Uuid);

impl TrainingProgramId {
    pub fn new(uuid: Uuid) -> Self {
        Self(uuid)
    }

    pub fn as_uuid(&self) -> Uuid {
        self.0
    }
}

impl From<Uuid> for TrainingProgramId {
    fn from(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

impl From<TrainingProgramId> for Uuid {
    fn from(id: TrainingProgramId) -> Uuid {
        id.0
    }
}
