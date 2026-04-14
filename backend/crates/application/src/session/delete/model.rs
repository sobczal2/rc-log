use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct DeleteSessionInput {
    pub id: Uuid,
    pub owner_id: Uuid,
}
