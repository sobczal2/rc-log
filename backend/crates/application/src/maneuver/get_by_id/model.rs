use rc_log_domain::maneuver::Maneuver;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::shared::difficulty::DifficultyDto;
use crate::shared::validator::{Validate, ValidationError};
use crate::shared::vehicle_type::VehicleTypeDto;

#[derive(Debug, Clone, Deserialize)]
pub struct GetManeuverByIdInput {
    pub id: Uuid,
}

impl Validate for GetManeuverByIdInput {
    fn validate(&self) -> Result<(), Vec<ValidationError>> {
        if self.id.is_nil() {
            return Err(vec![ValidationError::new("id", "must not be empty string (nil UUID)")]);
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TagDto {
    pub id: Uuid,
    pub name: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VariationDto {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub video_asset_name: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ManeuverDto {
    pub id: Uuid,
    pub vehicle_type: VehicleTypeDto,
    pub name: String,
    pub tags: Vec<TagDto>,
    pub description: String,
    pub difficulty: DifficultyDto,
    pub default_variation: VariationDto,
    pub variations: Vec<VariationDto>,
}

impl From<Maneuver> for ManeuverDto {
    fn from(m: Maneuver) -> Self {
        use rc_log_domain::shared::vehicle_type::VehicleType;
        let vehicle_type = match m.vehicle_type() {
            VehicleType::Helicopter => VehicleTypeDto::Helicopter,
            VehicleType::Plane => VehicleTypeDto::Plane,
            VehicleType::Drone => VehicleTypeDto::Drone,
        };

        use rc_log_domain::maneuver::difficulty::Difficulty;
        let difficulty = match m.difficulty() {
            Difficulty::Level1 => DifficultyDto::Level1,
            Difficulty::Level2 => DifficultyDto::Level2,
            Difficulty::Level3 => DifficultyDto::Level3,
            Difficulty::Level4 => DifficultyDto::Level4,
            Difficulty::Level5 => DifficultyDto::Level5,
            Difficulty::Level6 => DifficultyDto::Level6,
            Difficulty::Level7 => DifficultyDto::Level7,
        };

        let tags = m
            .tags()
            .iter()
            .map(|t| TagDto { id: Uuid::from(t.id()), name: t.name().to_string() })
            .collect();

        let default_variation = VariationDto {
            id: Uuid::from(m.default_variation().id()),
            name: m.default_variation().name().to_string(),
            description: m.default_variation().description().as_str().to_string(),
            video_asset_name: m.default_variation().video_asset_name().as_str().to_string(),
        };

        let variations = m
            .other_variations()
            .iter()
            .map(|v| VariationDto {
                id: Uuid::from(v.id()),
                name: v.name().to_string(),
                description: v.description().as_str().to_string(),
                video_asset_name: v.video_asset_name().as_str().to_string(),
            })
            .collect();

        Self {
            id: Uuid::from(m.id()),
            vehicle_type,
            name: m.name().to_string(),
            tags,
            description: m.description().as_str().to_string(),
            difficulty,
            default_variation,
            variations,
        }
    }
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use crate::shared::validator::Validate;

    use super::GetManeuverByIdInput;

    #[test]
    fn non_nil_uuid_passes_validation() {
        let input = GetManeuverByIdInput { id: Uuid::new_v4() };
        assert!(input.validate().is_ok());
    }

    #[test]
    fn nil_uuid_fails_validation() {
        let input = GetManeuverByIdInput { id: Uuid::nil() };
        let errs = input.validate().unwrap_err();
        assert_eq!(errs.len(), 1);
        assert_eq!(errs[0].field, "id");
    }
}
