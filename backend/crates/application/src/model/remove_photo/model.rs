use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct RemoveModelPhotoInput {
    pub model_id: Uuid,
    pub owner_id: Uuid,
}
