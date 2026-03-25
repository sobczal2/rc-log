use serde::Serialize;
use uuid::Uuid;

use rc_log_domain::maneuver::Maneuver;
use rc_log_domain::maneuver::difficulty::Difficulty;
use rc_log_domain::shared::vehicle_type::VehicleType;

#[derive(Debug, Serialize)]
pub struct TagResponse {
    pub id: Uuid,
    pub name: String,
}

#[derive(Debug, Serialize)]
pub struct GetManeuverByIdResponse {
    pub id: Uuid,
    pub vehicle_type: String,
    pub name: String,
    pub tags: Vec<TagResponse>,
    pub description: String,
    pub difficulty: u8,
    pub video_path: Option<String>,
}

impl From<Maneuver> for GetManeuverByIdResponse {
    fn from(maneuver: Maneuver) -> Self {
        let vehicle_type = match maneuver.vehicle_type() {
            VehicleType::Helicopter => "Helicopter",
            VehicleType::Plane => "Plane",
            VehicleType::Drone => "Drone",
        }
        .to_string();

        let difficulty = match maneuver.difficulty() {
            Difficulty::Level1 => 1,
            Difficulty::Level2 => 2,
            Difficulty::Level3 => 3,
            Difficulty::Level4 => 4,
            Difficulty::Level5 => 5,
            Difficulty::Level6 => 6,
            Difficulty::Level7 => 7,
        };

        let tags = maneuver
            .tags()
            .iter()
            .map(|t| TagResponse { id: t.id(), name: t.name().to_string() })
            .collect();

        let video_path = maneuver.video_path().map(|vp| vp.as_str().to_string());

        Self {
            id: maneuver.id(),
            vehicle_type,
            name: maneuver.name().to_string(),
            tags,
            description: maneuver.description().as_str().to_string(),
            difficulty,
            video_path,
        }
    }
}
