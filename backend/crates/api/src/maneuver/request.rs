use serde::Deserialize;
use rc_log_application::maneuver::list::model::{ListManeuversInput, ManeuverFilterDto, ManeuverSortDto};
use rc_log_application::shared::pagination::PaginationDto;

#[derive(Debug, Deserialize)]
pub struct ListManeuversQuery {
    pub page: Option<u32>,
    pub page_size: Option<u32>,
    pub tags: Option<String>,
    pub vehicle_type: Option<String>,
    pub difficulty: Option<u8>,
    pub search_query: Option<String>,
    pub sort: Option<String>,
}

impl ListManeuversQuery {
    pub fn into_dto(self) -> ListManeuversInput {
        let pagination = PaginationDto {
            page: self.page.unwrap_or(1).max(1),
            page_size: self.page_size.unwrap_or(20).clamp(1, 100),
        };

        let tags = self
            .tags
            .map(|s| s.split(',').map(|tag| tag.trim().to_string()).filter(|t| !t.is_empty()).collect())
            .unwrap_or_default();

        let filter = ManeuverFilterDto {
            tags,
            vehicle_type: self.vehicle_type,
            difficulty: self.difficulty,
            search_query: self.search_query,
        };

        let mut field = "name".to_string();
        let mut direction = "asc".to_string();

        if let Some(s) = self.sort {
            let parts: Vec<&str> = s.split('_').collect();
            if parts.len() == 2 {
                field = parts[0].to_string();
                direction = parts[1].to_string();
            } else {
                field = s;
            }
        }

        let sort = ManeuverSortDto { field, direction };

        ListManeuversInput {
            pagination,
            filter,
            sort,
        }
    }
}
