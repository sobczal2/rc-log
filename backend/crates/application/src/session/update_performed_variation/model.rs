use uuid::Uuid;

pub use crate::session::shared::rating::RatingDto;

#[derive(Debug, Clone)]
pub struct UpdatePerformedVariationInput {
    pub session_id: Uuid,
    pub owner_id: Uuid,
    pub performed_variation_id: Uuid,
    pub quality: RatingDto,
    pub comfort: RatingDto,
    pub repeatability: RatingDto,
    pub note: Option<String>,
}
