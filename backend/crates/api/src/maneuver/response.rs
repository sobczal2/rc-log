use rc_log_application::shared::paginated_result::PaginatedResult;
use rc_log_domain::maneuver::Maneuver;
use rc_log_domain::maneuver::difficulty::Difficulty;
use rc_log_domain::shared::vehicle_type::VehicleType;
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct TagResponse {
    pub id: Uuid,
    pub name: String,
}

fn vehicle_type_to_str(v: &VehicleType) -> &'static str {
    match v {
        VehicleType::Helicopter => "Helicopter",
        VehicleType::Plane => "Plane",
        VehicleType::Drone => "Drone",
    }
}

fn difficulty_to_u8(d: Difficulty) -> u8 {
    match d {
        Difficulty::Level1 => 1,
        Difficulty::Level2 => 2,
        Difficulty::Level3 => 3,
        Difficulty::Level4 => 4,
        Difficulty::Level5 => 5,
        Difficulty::Level6 => 6,
        Difficulty::Level7 => 7,
    }
}

// ── Get by id ────────────────────────────────────────────────────────────────

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
    fn from(m: Maneuver) -> Self {
        Self {
            id: m.id(),
            vehicle_type: vehicle_type_to_str(m.vehicle_type()).to_string(),
            name: m.name().to_string(),
            tags: m.tags().iter().map(|t| TagResponse { id: t.id(), name: t.name().to_string() }).collect(),
            description: m.description().as_str().to_string(),
            difficulty: difficulty_to_u8(m.difficulty()),
            video_path: m.video_path().map(|vp| vp.as_str().to_string()),
        }
    }
}

// ── List ─────────────────────────────────────────────────────────────────────

#[derive(Debug, Serialize)]
pub struct ManeuverListItemResponse {
    pub id: Uuid,
    pub vehicle_type: String,
    pub name: String,
    pub tags: Vec<TagResponse>,
    pub difficulty: u8,
    pub video_path: Option<String>,
}

impl From<Maneuver> for ManeuverListItemResponse {
    fn from(m: Maneuver) -> Self {
        Self {
            id: m.id(),
            vehicle_type: vehicle_type_to_str(m.vehicle_type()).to_string(),
            name: m.name().to_string(),
            tags: m.tags().iter().map(|t| TagResponse { id: t.id(), name: t.name().to_string() }).collect(),
            difficulty: difficulty_to_u8(m.difficulty()),
            video_path: m.video_path().map(|vp| vp.as_str().to_string()),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ListManeuversResponse {
    pub items: Vec<ManeuverListItemResponse>,
    pub total: u64,
    pub page: u32,
    pub page_size: u32,
    pub total_pages: u64,
}

impl From<PaginatedResult<Maneuver>> for ListManeuversResponse {
    fn from(result: PaginatedResult<Maneuver>) -> Self {
        let total_pages = result.total_pages();
        Self {
            total: result.total,
            page: result.page,
            page_size: result.page_size,
            total_pages,
            items: result.items.into_iter().map(ManeuverListItemResponse::from).collect(),
        }
    }
}
