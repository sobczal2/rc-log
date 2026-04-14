use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct RemovePerformedVariationInput {
    pub session_id: Uuid,
    pub owner_id: Uuid,
    pub performed_variation_id: Uuid,
}
