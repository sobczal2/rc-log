use rc_log_domain::training_program::TrainingProgram;
use rc_log_domain::training_program::part::{Part, PartVariation};
use serde::Serialize;
use specta::Type as SpectaType;
use uuid::Uuid;

use crate::shared::validator::{Validate, ValidationError};

#[derive(Debug, Clone)]
pub struct GetTrainingProgramByIdInput {
    pub id: Uuid,
}

impl Validate for GetTrainingProgramByIdInput {
    fn validate(&self) -> Result<(), Vec<ValidationError>> {
        if self.id.is_nil() {
            return Err(vec![ValidationError::new("id", "must not be a nil UUID")]);
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, SpectaType)]
#[serde(rename_all = "camelCase")]
pub struct PartVariationDto {
    pub variation_id: Uuid,
    pub position: u32,
}

#[derive(Debug, Clone, Serialize, SpectaType)]
#[serde(rename_all = "camelCase")]
pub struct PartDto {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub position: u32,
    pub variations: Vec<PartVariationDto>,
}

#[derive(Debug, Clone, Serialize, SpectaType)]
#[serde(rename_all = "camelCase")]
pub struct TrainingProgramDto {
    pub id: Uuid,
    pub author_id: Uuid,
    pub name: String,
    pub description: String,
    pub parts: Vec<PartDto>,
}

impl From<PartVariation> for PartVariationDto {
    fn from(pv: PartVariation) -> Self {
        Self {
            variation_id: Uuid::from(pv.variation_id()),
            position: pv.position(),
        }
    }
}

impl From<Part> for PartDto {
    fn from(p: Part) -> Self {
        Self {
            id: Uuid::from(p.id()),
            name: p.name().as_str().to_string(),
            description: p.description().as_str().to_string(),
            position: p.position(),
            variations: p.variations().iter().cloned().map(PartVariationDto::from).collect(),
        }
    }
}

impl From<TrainingProgram> for TrainingProgramDto {
    fn from(tp: TrainingProgram) -> Self {
        Self {
            id: Uuid::from(tp.id()),
            author_id: Uuid::from(tp.author_id()),
            name: tp.name().as_str().to_string(),
            description: tp.description().as_str().to_string(),
            parts: tp.parts().iter().cloned().map(PartDto::from).collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use crate::shared::validator::Validate;

    use super::GetTrainingProgramByIdInput;

    #[test]
    fn non_nil_uuid_passes_validation() {
        let input = GetTrainingProgramByIdInput { id: Uuid::new_v4() };
        assert!(input.validate().is_ok());
    }

    #[test]
    fn nil_uuid_fails_validation() {
        let input = GetTrainingProgramByIdInput { id: Uuid::nil() };
        let errs = input.validate().unwrap_err();
        assert_eq!(errs.len(), 1);
        assert_eq!(errs[0].field, "id");
    }
}
