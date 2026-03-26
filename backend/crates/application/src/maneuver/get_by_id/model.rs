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
pub struct ManeuverDto {
    pub id: Uuid,
    pub vehicle_type: VehicleTypeDto,
    pub name: String,
    pub tags: Vec<TagDto>,
    pub description: String,
    pub difficulty: DifficultyDto,
    pub video_path: Option<String>,
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

        let tags =
            m.tags().iter().map(|t| TagDto { id: t.id(), name: t.name().to_string() }).collect();

        Self {
            id: m.id(),
            vehicle_type,
            name: m.name().to_string(),
            tags,
            description: m.description().as_str().to_string(),
            difficulty,
            video_path: m.video_path().map(|vp| vp.as_str().to_string()),
        }
    }
}
