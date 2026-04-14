pub mod id;

use crate::maneuver::variation::VariationId;
use crate::session::performed_variation::id::PerformedVariationId;
use crate::session::rating::Rating;
use crate::shared::markdown_text::MarkdownText;

#[derive(Debug, Clone)]
pub struct PerformedVariation {
    id: PerformedVariationId,
    variation_id: VariationId,
    quality: Rating,
    comfort: Rating,
    repeatability: Rating,
    note: Option<MarkdownText>,
}

impl PerformedVariation {
    pub fn new(
        id: PerformedVariationId,
        variation_id: VariationId,
        quality: Rating,
        comfort: Rating,
        repeatability: Rating,
        note: Option<MarkdownText>,
    ) -> Self {
        Self { id, variation_id, quality, comfort, repeatability, note }
    }

    pub fn id(&self) -> PerformedVariationId {
        self.id
    }

    pub fn variation_id(&self) -> VariationId {
        self.variation_id
    }

    pub fn quality(&self) -> Rating {
        self.quality
    }

    pub fn comfort(&self) -> Rating {
        self.comfort
    }

    pub fn repeatability(&self) -> Rating {
        self.repeatability
    }

    pub fn note(&self) -> Option<&MarkdownText> {
        self.note.as_ref()
    }
}
