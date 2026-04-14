pub mod date;
pub mod id;
pub mod performed_variation;
pub mod rating;
pub mod transaction;

use crate::model::id::ModelId;
use crate::session::date::Date;
use crate::session::id::SessionId;
use crate::session::performed_variation::PerformedVariation;
use crate::shared::markdown_text::MarkdownText;
use crate::user::id::UserId;

#[derive(Debug, Clone)]
pub struct Session {
    id: SessionId,
    user_id: UserId,
    date: Date,
    model_id: Option<ModelId>,
    note: Option<MarkdownText>,
    performed_variations: Vec<PerformedVariation>,
}

impl Session {
    pub fn new(
        id: SessionId,
        user_id: UserId,
        date: Date,
        model_id: Option<ModelId>,
        note: Option<MarkdownText>,
        performed_variations: Vec<PerformedVariation>,
    ) -> Self {
        Self { id, user_id, date, model_id, note, performed_variations }
    }

    pub fn id(&self) -> SessionId {
        self.id
    }

    pub fn user_id(&self) -> UserId {
        self.user_id
    }

    pub fn date(&self) -> &Date {
        &self.date
    }

    pub fn model_id(&self) -> Option<ModelId> {
        self.model_id
    }

    pub fn note(&self) -> Option<&MarkdownText> {
        self.note.as_ref()
    }

    pub fn performed_variations(&self) -> &[PerformedVariation] {
        &self.performed_variations
    }
}
