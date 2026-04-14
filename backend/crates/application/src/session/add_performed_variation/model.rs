use rc_log_domain::session::performed_variation::PerformedVariation;
use serde::Serialize;
use specta::Type;
use uuid::Uuid;

use crate::session::shared::rating::rating_to_dto;

pub use crate::session::shared::rating::RatingDto;

#[derive(Debug, Clone)]
pub struct AddPerformedVariationInput {
    pub session_id: Uuid,
    pub owner_id: Uuid,
    pub variation_id: Uuid,
    pub quality: RatingDto,
    pub comfort: RatingDto,
    pub repeatability: RatingDto,
    pub note: Option<String>,
}

#[derive(Debug, Clone, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct PerformedVariationDto {
    pub performed_variation_id: Uuid,
    pub variation_id: Uuid,
    pub quality: RatingDto,
    pub comfort: RatingDto,
    pub repeatability: RatingDto,
    pub note: Option<String>,
}

impl From<PerformedVariation> for PerformedVariationDto {
    fn from(performed: PerformedVariation) -> Self {
        Self {
            performed_variation_id: Uuid::from(performed.id()),
            variation_id: Uuid::from(performed.variation_id()),
            quality: rating_to_dto(performed.quality()),
            comfort: rating_to_dto(performed.comfort()),
            repeatability: rating_to_dto(performed.repeatability()),
            note: performed.note().map(|n| n.as_str().to_string()),
        }
    }
}
