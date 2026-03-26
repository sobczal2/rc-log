use rc_log_domain::maneuver::Maneuver;
use rc_log_domain::maneuver::difficulty::Difficulty;
use rc_log_domain::shared::vehicle_type::VehicleType;
use serde::Serialize;
use uuid::Uuid;

use rc_log_domain::maneuver::query::{
    ManeuverFilter, ManeuverSort, ManeuverSortField, SortDirection,
};

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

#[derive(Debug, Clone, Default)]
pub struct ManeuverFilterDto {
    pub tags: Vec<String>,
    pub vehicle_type: Option<String>,
    pub difficulty: Option<u8>,
    pub search_query: Option<String>,
}

impl From<ManeuverFilterDto> for ManeuverFilter {
    fn from(dto: ManeuverFilterDto) -> Self {
        let vehicle_type = match dto.vehicle_type.as_deref() {
            Some("Helicopter") => Some(VehicleType::Helicopter),
            Some("Plane") => Some(VehicleType::Plane),
            Some("Drone") => Some(VehicleType::Drone),
            _ => None,
        };

        let difficulty = match dto.difficulty {
            Some(1) => Some(Difficulty::Level1),
            Some(2) => Some(Difficulty::Level2),
            Some(3) => Some(Difficulty::Level3),
            Some(4) => Some(Difficulty::Level4),
            Some(5) => Some(Difficulty::Level5),
            Some(6) => Some(Difficulty::Level6),
            Some(7) => Some(Difficulty::Level7),
            _ => None,
        };

        Self { tags: dto.tags, vehicle_type, difficulty, search_query: dto.search_query }
    }
}

#[derive(Debug, Clone)]
pub struct ManeuverSortDto {
    pub field: String,
    pub direction: String,
}

impl Default for ManeuverSortDto {
    fn default() -> Self {
        Self { field: "name".to_string(), direction: "asc".to_string() }
    }
}

impl From<ManeuverSortDto> for ManeuverSort {
    fn from(dto: ManeuverSortDto) -> Self {
        let field = match dto.field.to_lowercase().as_str() {
            "difficulty" => ManeuverSortField::Difficulty,
            _ => ManeuverSortField::Name,
        };

        let direction = match dto.direction.to_lowercase().as_str() {
            "desc" => SortDirection::Desc,
            _ => SortDirection::Asc,
        };

        Self { field, direction }
    }
}

use crate::shared::pagination::PaginationDto;

#[derive(Debug, Clone)]
pub struct ListManeuversInput {
    pub pagination: PaginationDto,
    pub filter: ManeuverFilterDto,
    pub sort: ManeuverSortDto,
}
