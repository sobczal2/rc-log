use rc_log_domain::maneuver::Maneuver;
use serde::Serialize;
use uuid::Uuid;

use crate::shared::difficulty::DifficultyDto;
use crate::shared::pagination::PaginationDto;
use crate::shared::validator::{Validate, ValidationError};
use crate::shared::vehicle_type::VehicleTypeDto;

use rc_log_domain::maneuver::transaction::{
    ManeuverFilter, ManeuverSort, ManeuverSortField, SortDirection,
};

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
    pub default_variation_video_asset_name: String,
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
            default_variation_video_asset_name: m
                .default_variation()
                .video_asset_name()
                .as_str()
                .to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ManeuverFilterDto {
    pub tags: Vec<String>,
    pub vehicle_type: Option<VehicleTypeDto>,
    pub difficulty: Option<DifficultyDto>,
    pub search_query: Option<String>,
}

impl Validate for ManeuverFilterDto {
    fn validate(&self) -> Result<(), Vec<ValidationError>> {
        let mut errors = Vec::new();
        if let Some(sq) = &self.search_query {
            if sq.len() > 100 {
                errors.push(ValidationError::new("search_query", "must not exceed 100 characters"));
            }
        }
        for (i, tag) in self.tags.iter().enumerate() {
            if tag.len() > 50 {
                errors.push(ValidationError::new(
                    format!("tags[{}]", i),
                    "must not exceed 50 characters",
                ));
            }
        }
        if errors.is_empty() { Ok(()) } else { Err(errors) }
    }
}

impl From<ManeuverFilterDto> for ManeuverFilter {
    fn from(dto: ManeuverFilterDto) -> Self {
        use rc_log_domain::shared::vehicle_type::VehicleType;
        let vehicle_type = match dto.vehicle_type {
            Some(VehicleTypeDto::Helicopter) => Some(VehicleType::Helicopter),
            Some(VehicleTypeDto::Plane) => Some(VehicleType::Plane),
            Some(VehicleTypeDto::Drone) => Some(VehicleType::Drone),
            None => None,
        };

        use rc_log_domain::maneuver::difficulty::Difficulty;
        let difficulty = match dto.difficulty {
            Some(DifficultyDto::Level1) => Some(Difficulty::Level1),
            Some(DifficultyDto::Level2) => Some(Difficulty::Level2),
            Some(DifficultyDto::Level3) => Some(Difficulty::Level3),
            Some(DifficultyDto::Level4) => Some(Difficulty::Level4),
            Some(DifficultyDto::Level5) => Some(Difficulty::Level5),
            Some(DifficultyDto::Level6) => Some(Difficulty::Level6),
            Some(DifficultyDto::Level7) => Some(Difficulty::Level7),
            None => None,
        };

        Self { tags: dto.tags, vehicle_type, difficulty, search_query: dto.search_query }
    }
}

#[derive(Debug, Clone)]
pub struct ManeuverSortDto {
    pub field: String,
    pub direction: String,
}

impl Validate for ManeuverSortDto {
    fn validate(&self) -> Result<(), Vec<ValidationError>> {
        let mut errors = Vec::new();
        let valid_fields = ["", "name", "difficulty"];
        if !valid_fields.contains(&self.field.as_str()) {
            errors.push(ValidationError::new("sort.field", "must be 'name' or 'difficulty'"));
        }
        let valid_dirs = ["", "asc", "desc"];
        if !valid_dirs.contains(&self.direction.as_str()) {
            errors.push(ValidationError::new("sort.direction", "must be 'asc' or 'desc'"));
        }
        if errors.is_empty() { Ok(()) } else { Err(errors) }
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

#[derive(Debug, Clone)]
pub struct ListManeuversInput {
    pub pagination: PaginationDto,
    pub filter: ManeuverFilterDto,
    pub sort: ManeuverSortDto,
}

impl Validate for ListManeuversInput {
    fn validate(&self) -> Result<(), Vec<ValidationError>> {
        let mut errors = Vec::new();
        if let Err(mut e) = self.pagination.validate() {
            errors.append(&mut e);
        }
        if let Err(mut e) = self.filter.validate() {
            errors.append(&mut e);
        }
        if let Err(mut e) = self.sort.validate() {
            errors.append(&mut e);
        }
        if errors.is_empty() { Ok(()) } else { Err(errors) }
    }
}

#[cfg(test)]
mod tests {
    use crate::shared::pagination::PaginationDto;
    use crate::shared::validator::Validate;

    use super::{ListManeuversInput, ManeuverFilterDto, ManeuverSortDto};

    fn valid_filter() -> ManeuverFilterDto {
        ManeuverFilterDto { tags: vec![], vehicle_type: None, difficulty: None, search_query: None }
    }

    fn valid_sort() -> ManeuverSortDto {
        ManeuverSortDto { field: String::new(), direction: String::new() }
    }

    fn valid_pagination() -> PaginationDto {
        PaginationDto { page: 1, page_size: 20 }
    }

    // --- ManeuverFilterDto ---

    #[test]
    fn filter_empty_passes() {
        assert!(valid_filter().validate().is_ok());
    }

    #[test]
    fn filter_search_query_100_chars_passes() {
        let mut f = valid_filter();
        f.search_query = Some("a".repeat(100));
        assert!(f.validate().is_ok());
    }

    #[test]
    fn filter_search_query_101_chars_fails() {
        let mut f = valid_filter();
        f.search_query = Some("a".repeat(101));
        let errs = f.validate().unwrap_err();
        assert_eq!(errs.len(), 1);
        assert_eq!(errs[0].field, "search_query");
    }

    #[test]
    fn filter_tag_50_chars_passes() {
        let mut f = valid_filter();
        f.tags = vec!["a".repeat(50)];
        assert!(f.validate().is_ok());
    }

    #[test]
    fn filter_tag_51_chars_fails() {
        let mut f = valid_filter();
        f.tags = vec!["a".repeat(51)];
        let errs = f.validate().unwrap_err();
        assert_eq!(errs.len(), 1);
        assert_eq!(errs[0].field, "tags[0]");
    }

    #[test]
    fn filter_multiple_bad_tags_report_all_errors() {
        let mut f = valid_filter();
        f.tags = vec!["a".repeat(51), "b".repeat(51)];
        assert_eq!(f.validate().unwrap_err().len(), 2);
    }

    // --- ManeuverSortDto ---

    #[test]
    fn sort_empty_strings_pass() {
        assert!(valid_sort().validate().is_ok());
    }

    #[test]
    fn sort_name_asc_passes() {
        let s = ManeuverSortDto { field: "name".to_string(), direction: "asc".to_string() };
        assert!(s.validate().is_ok());
    }

    #[test]
    fn sort_difficulty_desc_passes() {
        let s = ManeuverSortDto { field: "difficulty".to_string(), direction: "desc".to_string() };
        assert!(s.validate().is_ok());
    }

    #[test]
    fn sort_invalid_field_fails() {
        let s = ManeuverSortDto { field: "bogus".to_string(), direction: "asc".to_string() };
        let errs = s.validate().unwrap_err();
        assert_eq!(errs.len(), 1);
        assert_eq!(errs[0].field, "sort.field");
    }

    #[test]
    fn sort_invalid_direction_fails() {
        let s = ManeuverSortDto { field: "name".to_string(), direction: "sideways".to_string() };
        let errs = s.validate().unwrap_err();
        assert_eq!(errs.len(), 1);
        assert_eq!(errs[0].field, "sort.direction");
    }

    #[test]
    fn sort_both_invalid_reports_two_errors() {
        let s = ManeuverSortDto { field: "bad".to_string(), direction: "bad".to_string() };
        assert_eq!(s.validate().unwrap_err().len(), 2);
    }

    // --- ListManeuversInput ---

    #[test]
    fn list_input_all_valid_passes() {
        let input = ListManeuversInput {
            pagination: valid_pagination(),
            filter: valid_filter(),
            sort: valid_sort(),
        };
        assert!(input.validate().is_ok());
    }

    #[test]
    fn list_input_invalid_pagination_propagates_errors() {
        let input = ListManeuversInput {
            pagination: PaginationDto { page: 0, page_size: 0 },
            filter: valid_filter(),
            sort: valid_sort(),
        };
        let errs = input.validate().unwrap_err();
        // page and page_size both invalid
        assert!(errs.len() >= 2);
    }

    #[test]
    fn list_input_invalid_filter_propagates_errors() {
        let mut filter = valid_filter();
        filter.search_query = Some("x".repeat(101));
        let input = ListManeuversInput {
            pagination: valid_pagination(),
            filter,
            sort: valid_sort(),
        };
        assert!(input.validate().is_err());
    }
}
