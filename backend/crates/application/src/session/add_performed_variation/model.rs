use rc_log_domain::session::performed_variation::PerformedVariation;
use serde::Serialize;
use specta::Type;
use uuid::Uuid;

use crate::session::shared::rating::{comfort_to_dto, quality_to_dto, repeatability_to_dto};

pub use crate::session::shared::rating::{ComfortDto, QualityDto, RepeatabilityDto};

#[derive(Debug, Clone)]
pub struct AddPerformedVariationInput {
    pub session_id: Uuid,
    pub owner_id: Uuid,
    pub variation_id: Uuid,
    pub quality: QualityDto,
    pub comfort: ComfortDto,
    pub repeatability: RepeatabilityDto,
    pub note: Option<String>,
}

#[derive(Debug, Clone, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct PerformedVariationDto {
    pub performed_variation_id: Uuid,
    pub variation_id: Uuid,
    pub quality: QualityDto,
    pub comfort: ComfortDto,
    pub repeatability: RepeatabilityDto,
    pub note: Option<String>,
}

impl From<PerformedVariation> for PerformedVariationDto {
    fn from(performed: PerformedVariation) -> Self {
        let rating = performed.rating();
        Self {
            performed_variation_id: Uuid::from(performed.id()),
            variation_id: Uuid::from(performed.variation_id()),
            quality: quality_to_dto(rating.quality()),
            comfort: comfort_to_dto(rating.comfort()),
            repeatability: repeatability_to_dto(rating.repeatability()),
            note: performed.note().map(|n| n.as_str().to_string()),
        }
    }
}
