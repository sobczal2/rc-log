pub mod id;

use crate::maneuver::variation::VariationId;
use crate::shared::markdown_text::MarkdownText;
use crate::training_program::id::TrainingProgramId;
use crate::training_program::name::TrainingProgramName;
use crate::training_program::part::id::TrainingProgramPartId;

#[derive(Debug, Clone)]
pub struct TrainingProgramPartVariation {
    variation_id: VariationId,
    position: u32,
}

impl TrainingProgramPartVariation {
    pub fn new(variation_id: VariationId, position: u32) -> Self {
        Self { variation_id, position }
    }

    pub fn variation_id(&self) -> VariationId {
        self.variation_id
    }

    pub fn position(&self) -> u32 {
        self.position
    }
}

#[derive(Debug, Clone)]
pub struct TrainingProgramPart {
    id: TrainingProgramPartId,
    training_program_id: TrainingProgramId,
    name: TrainingProgramName,
    description: MarkdownText,
    position: u32,
    variations: Vec<TrainingProgramPartVariation>,
}

impl TrainingProgramPart {
    pub fn new(
        id: TrainingProgramPartId,
        training_program_id: TrainingProgramId,
        name: TrainingProgramName,
        description: MarkdownText,
        position: u32,
        variations: Vec<TrainingProgramPartVariation>,
    ) -> Self {
        Self { id, training_program_id, name, description, position, variations }
    }

    pub fn id(&self) -> TrainingProgramPartId {
        self.id
    }

    pub fn training_program_id(&self) -> TrainingProgramId {
        self.training_program_id
    }

    pub fn name(&self) -> &TrainingProgramName {
        &self.name
    }

    pub fn description(&self) -> &MarkdownText {
        &self.description
    }

    pub fn position(&self) -> u32 {
        self.position
    }

    pub fn variations(&self) -> &[TrainingProgramPartVariation] {
        &self.variations
    }
}
