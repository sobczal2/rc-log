pub mod id;
pub mod name;
pub mod part;
pub mod transaction;

use crate::shared::markdown_text::MarkdownText;
use crate::training_program::id::TrainingProgramId;
use crate::training_program::name::TrainingProgramName;
use crate::training_program::part::TrainingProgramPart;
use crate::user::id::UserId;

#[derive(Debug, Clone)]
pub struct TrainingProgram {
    id: TrainingProgramId,
    author_id: UserId,
    name: TrainingProgramName,
    description: MarkdownText,
    parts: Vec<TrainingProgramPart>,
}

impl TrainingProgram {
    pub fn new(
        id: TrainingProgramId,
        author_id: UserId,
        name: TrainingProgramName,
        description: MarkdownText,
        parts: Vec<TrainingProgramPart>,
    ) -> Self {
        Self { id, author_id, name, description, parts }
    }

    pub fn id(&self) -> TrainingProgramId {
        self.id
    }

    pub fn author_id(&self) -> UserId {
        self.author_id
    }

    pub fn name(&self) -> &TrainingProgramName {
        &self.name
    }

    pub fn description(&self) -> &MarkdownText {
        &self.description
    }

    pub fn parts(&self) -> &[TrainingProgramPart] {
        &self.parts
    }
}
