use rc_log_domain::session::performed_variation::PerformedVariation;
use rc_log_domain::session::rating::{Comfort, Quality, Repeatability};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum QualityDto {
    One,
    Two,
    Three,
    Four,
    Five,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum ComfortDto {
    One,
    Two,
    Three,
    Four,
    Five,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum RepeatabilityDto {
    One,
    Two,
    Three,
    Four,
    Five,
}

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

#[derive(Debug, Clone, Serialize)]
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

fn quality_to_dto(quality: Quality) -> QualityDto {
    match quality {
        Quality::One => QualityDto::One,
        Quality::Two => QualityDto::Two,
        Quality::Three => QualityDto::Three,
        Quality::Four => QualityDto::Four,
        Quality::Five => QualityDto::Five,
    }
}

fn comfort_to_dto(comfort: Comfort) -> ComfortDto {
    match comfort {
        Comfort::One => ComfortDto::One,
        Comfort::Two => ComfortDto::Two,
        Comfort::Three => ComfortDto::Three,
        Comfort::Four => ComfortDto::Four,
        Comfort::Five => ComfortDto::Five,
    }
}

fn repeatability_to_dto(repeatability: Repeatability) -> RepeatabilityDto {
    match repeatability {
        Repeatability::One => RepeatabilityDto::One,
        Repeatability::Two => RepeatabilityDto::Two,
        Repeatability::Three => RepeatabilityDto::Three,
        Repeatability::Four => RepeatabilityDto::Four,
        Repeatability::Five => RepeatabilityDto::Five,
    }
}
