use rc_log_domain::asset::photo::Photo;
use serde::Serialize;

use crate::shared::validator::{Validate, ValidationError};

#[derive(Debug, Clone)]
pub struct ResolvePhotoInput {
    pub name: String,
}

impl Validate for ResolvePhotoInput {
    fn validate(&self) -> Result<(), Vec<ValidationError>> {
        let mut errors = Vec::new();
        if self.name.trim().is_empty() {
            errors.push(ValidationError::new("name", "must not be empty"));
        }
        if self.name.len() > 255 {
            errors.push(ValidationError::new("name", "must not exceed 255 characters"));
        }
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PhotoPathsDto {
    pub name: String,
    pub small_path: String,
    pub medium_path: Option<String>,
    pub large_path: Option<String>,
}

impl From<Photo> for PhotoPathsDto {
    fn from(p: Photo) -> Self {
        Self {
            name: p.name.as_str().to_string(),
            small_path: p.small_path.as_str().to_string(),
            medium_path: p.medium_path.as_ref().map(|path| path.as_str().to_string()),
            large_path: p.large_path.as_ref().map(|path| path.as_str().to_string()),
        }
    }
}
