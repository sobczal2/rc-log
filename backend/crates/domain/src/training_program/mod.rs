pub mod id;
pub mod name;
pub mod part;
pub mod transaction;

use crate::shared::markdown_text::MarkdownText;
use crate::training_program::id::TrainingProgramId;
use crate::training_program::name::Name;
use crate::training_program::part::Part;
use crate::user::id::UserId;

#[derive(Debug, Clone)]
pub struct TrainingProgram {
    id: TrainingProgramId,
    author_id: UserId,
    name: Name,
    description: MarkdownText,
    parts: Vec<Part>,
}

impl TrainingProgram {
    pub fn new(
        id: TrainingProgramId,
        author_id: UserId,
        name: Name,
        description: MarkdownText,
        parts: Vec<Part>,
    ) -> Self {
        Self { id, author_id, name, description, parts }
    }

    pub fn id(&self) -> TrainingProgramId {
        self.id
    }

    pub fn author_id(&self) -> UserId {
        self.author_id
    }

    pub fn name(&self) -> &Name {
        &self.name
    }

    pub fn description(&self) -> &MarkdownText {
        &self.description
    }

    pub fn parts(&self) -> &[Part] {
        &self.parts
    }
}
