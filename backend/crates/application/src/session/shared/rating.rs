use rc_log_domain::session::rating::{Comfort, Quality, Repeatability};
use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Debug, Clone, Serialize, Deserialize, Type, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum QualityDto {
    One,
    Two,
    Three,
    Four,
    Five,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum ComfortDto {
    One,
    Two,
    Three,
    Four,
    Five,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum RepeatabilityDto {
    One,
    Two,
    Three,
    Four,
    Five,
}

pub fn quality_to_dto(quality: Quality) -> QualityDto {
    match quality {
        Quality::One => QualityDto::One,
        Quality::Two => QualityDto::Two,
        Quality::Three => QualityDto::Three,
        Quality::Four => QualityDto::Four,
        Quality::Five => QualityDto::Five,
    }
}

pub fn comfort_to_dto(comfort: Comfort) -> ComfortDto {
    match comfort {
        Comfort::One => ComfortDto::One,
        Comfort::Two => ComfortDto::Two,
        Comfort::Three => ComfortDto::Three,
        Comfort::Four => ComfortDto::Four,
        Comfort::Five => ComfortDto::Five,
    }
}

pub fn repeatability_to_dto(repeatability: Repeatability) -> RepeatabilityDto {
    match repeatability {
        Repeatability::One => RepeatabilityDto::One,
        Repeatability::Two => RepeatabilityDto::Two,
        Repeatability::Three => RepeatabilityDto::Three,
        Repeatability::Four => RepeatabilityDto::Four,
        Repeatability::Five => RepeatabilityDto::Five,
    }
}

pub fn quality_from_dto(quality: QualityDto) -> Quality {
    match quality {
        QualityDto::One => Quality::One,
        QualityDto::Two => Quality::Two,
        QualityDto::Three => Quality::Three,
        QualityDto::Four => Quality::Four,
        QualityDto::Five => Quality::Five,
    }
}

pub fn comfort_from_dto(comfort: ComfortDto) -> Comfort {
    match comfort {
        ComfortDto::One => Comfort::One,
        ComfortDto::Two => Comfort::Two,
        ComfortDto::Three => Comfort::Three,
        ComfortDto::Four => Comfort::Four,
        ComfortDto::Five => Comfort::Five,
    }
}

pub fn repeatability_from_dto(repeatability: RepeatabilityDto) -> Repeatability {
    match repeatability {
        RepeatabilityDto::One => Repeatability::One,
        RepeatabilityDto::Two => Repeatability::Two,
        RepeatabilityDto::Three => Repeatability::Three,
        RepeatabilityDto::Four => Repeatability::Four,
        RepeatabilityDto::Five => Repeatability::Five,
    }
}
