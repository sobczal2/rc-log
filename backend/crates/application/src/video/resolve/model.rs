use rc_log_domain::asset::video::Video;
use serde::Serialize;

use crate::shared::validator::{Validate, ValidationError};

#[derive(Debug, Clone)]
pub struct ResolveVideoInput {
    pub name: String,
}

impl Validate for ResolveVideoInput {
    fn validate(&self) -> Result<(), Vec<ValidationError>> {
        let mut errors = Vec::new();
        if self.name.trim().is_empty() {
            errors.push(ValidationError::new("name", "must not be empty"));
        }
        if self.name.len() > 255 {
            errors.push(ValidationError::new("name", "must not exceed 255 characters"));
        }
        if errors.is_empty() { Ok(()) } else { Err(errors) }
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoPathsDto {
    pub name: String,
    pub small_path: String,
    pub medium_path: Option<String>,
    pub large_path: Option<String>,
}

impl From<Video> for VideoPathsDto {
    fn from(v: Video) -> Self {
        Self {
            name: v.name.as_str().to_string(),
            small_path: v.small_path.as_str().to_string(),
            medium_path: v.medium_path.as_ref().map(|p| p.as_str().to_string()),
            large_path: v.large_path.as_ref().map(|p| p.as_str().to_string()),
        }
    }
}
