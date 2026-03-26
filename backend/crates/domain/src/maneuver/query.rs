use crate::shared::vehicle_type::VehicleType;
use crate::maneuver::difficulty::Difficulty;

#[derive(Debug, Clone, Default)]
pub struct ManeuverFilter {
    pub tags: Vec<String>,
    pub vehicle_type: Option<VehicleType>,
    pub difficulty: Option<Difficulty>,
    pub search_query: Option<String>,
}

#[derive(Debug, Clone, Copy)]
pub enum ManeuverSortField {
    Name,
    Difficulty,
}

#[derive(Debug, Clone, Copy)]
pub enum SortDirection {
    Asc,
    Desc,
}

#[derive(Debug, Clone, Copy)]
pub struct ManeuverSort {
    pub field: ManeuverSortField,
    pub direction: SortDirection,
}

impl Default for ManeuverSort {
    fn default() -> Self {
        Self { field: ManeuverSortField::Name, direction: SortDirection::Asc }
    }
}
