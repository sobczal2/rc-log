use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct DeleteModelInput {
    pub id: Uuid,
    pub owner_id: Uuid,
}
