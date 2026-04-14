use rc_log_domain::asset::video::Video;
use serde::Serialize;
use specta::Type;

use crate::shared::validator::{Validate, ValidationError};

#[derive(Debug, Clone)]
pub struct ResolveVideoInput {
    pub id: String,
}

impl Validate for ResolveVideoInput {
    fn validate(&self) -> Result<(), Vec<ValidationError>> {
        let mut errors = Vec::new();
        if self.id.trim().is_empty() {
            errors.push(ValidationError::new("id", "must not be empty"));
        }
        if uuid::Uuid::parse_str(&self.id).is_err() {
            errors.push(ValidationError::new("id", "must be a valid UUID"));
        }
        if errors.is_empty() { Ok(()) } else { Err(errors) }
    }
}

#[derive(Debug, Clone, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct VideoPathsDto {
    pub id: String,
    pub small_path: String,
    pub medium_path: Option<String>,
    pub large_path: Option<String>,
}

impl From<Video> for VideoPathsDto {
    fn from(v: Video) -> Self {
        Self {
            id: v.id.as_uuid().to_string(),
            small_path: v.small_path.as_str().to_string(),
            medium_path: v.medium_path.as_ref().map(|p| p.as_str().to_string()),
            large_path: v.large_path.as_ref().map(|p| p.as_str().to_string()),
        }
    }
}
