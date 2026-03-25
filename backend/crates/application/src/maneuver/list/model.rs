use rc_log_domain::maneuver::Maneuver;
use rc_log_domain::maneuver::difficulty::Difficulty;
use rc_log_domain::shared::vehicle_type::VehicleType;
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize)]
pub struct TagDto {
    pub id: Uuid,
    pub name: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ManeuverDto {
    pub id: Uuid,
    pub vehicle_type: String,
    pub name: String,
    pub tags: Vec<TagDto>,
    pub description: String,
    pub difficulty: u8,
    pub video_path: Option<String>,
}

impl From<Maneuver> for ManeuverDto {
    fn from(m: Maneuver) -> Self {
        let vehicle_type = match m.vehicle_type() {
            VehicleType::Helicopter => "Helicopter",
            VehicleType::Plane => "Plane",
            VehicleType::Drone => "Drone",
        }
        .to_string();

        let difficulty = match m.difficulty() {
            Difficulty::Level1 => 1,
            Difficulty::Level2 => 2,
            Difficulty::Level3 => 3,
            Difficulty::Level4 => 4,
            Difficulty::Level5 => 5,
            Difficulty::Level6 => 6,
            Difficulty::Level7 => 7,
        };

        let tags = m
            .tags()
            .iter()
            .map(|t| TagDto { id: t.id(), name: t.name().to_string() })
            .collect();

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
