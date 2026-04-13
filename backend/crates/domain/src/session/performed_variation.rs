use crate::maneuver::variation::VariationId;
use crate::session::rating::Rating;
use crate::shared::markdown_text::MarkdownText;

#[derive(Debug, Clone)]
pub struct PerformedVariation {
    variation_id: VariationId,
    rating: Rating,
    note: Option<MarkdownText>,
}

impl PerformedVariation {
    pub fn new(
        variation_id: VariationId,
        rating: Rating,
        note: Option<MarkdownText>,
    ) -> Self {
        Self { variation_id, rating, note }
    }

    pub fn variation_id(&self) -> VariationId {
        self.variation_id
    }

    pub fn rating(&self) -> Rating {
        self.rating
    }

    pub fn note(&self) -> Option<&MarkdownText> {
        self.note.as_ref()
    }
}