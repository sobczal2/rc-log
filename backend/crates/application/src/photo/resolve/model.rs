use rc_log_domain::asset::photo::Photo;
use serde::Serialize;
use specta::Type;

use crate::shared::validator::{Validate, ValidationError};

#[derive(Debug, Clone)]
pub struct ResolvePhotoInput {
    pub id: String,
}

impl Validate for ResolvePhotoInput {
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
pub struct PhotoPathsDto {
    pub id: String,
    pub small_path: String,
    pub medium_path: Option<String>,
    pub large_path: Option<String>,
}

impl From<Photo> for PhotoPathsDto {
    fn from(p: Photo) -> Self {
        Self {
            id: p.id.as_uuid().to_string(),
            small_path: p.small_path.as_str().to_string(),
            medium_path: p.medium_path.as_ref().map(|path| path.as_str().to_string()),
            large_path: p.large_path.as_ref().map(|path| path.as_str().to_string()),
        }
    }
}
