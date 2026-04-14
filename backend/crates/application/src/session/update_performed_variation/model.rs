use uuid::Uuid;

pub use crate::session::shared::rating::{ComfortDto, QualityDto, RepeatabilityDto};

#[derive(Debug, Clone)]
pub struct UpdatePerformedVariationInput {
    pub session_id: Uuid,
    pub owner_id: Uuid,
    pub performed_variation_id: Uuid,
    pub quality: QualityDto,
    pub comfort: ComfortDto,
    pub repeatability: RepeatabilityDto,
    pub note: Option<String>,
}
