pub mod difficulty;
pub mod tag;
pub mod transaction;
pub mod variation;

use std::collections::BTreeSet;

use uuid::Uuid;

use crate::{
    maneuver::{difficulty::Difficulty, tag::Tag, variation::Variation},
    shared::{markdown_text::MarkdownText, vehicle_type::VehicleType},
};

#[derive(Debug, Clone)]
pub struct Maneuver {
    id: Uuid,
    vehicle_type: VehicleType,
    name: String,
    tags: BTreeSet<Tag>,
    description: MarkdownText,
    difficulty: Difficulty,
    default_variation: Variation,
    other_variations: Vec<Variation>,
}

impl Maneuver {
    pub fn new(
        id: Uuid,
        vehicle_type: VehicleType,
        name: String,
        tags: BTreeSet<Tag>,
        description: MarkdownText,
        difficulty: Difficulty,
        default_variation: Variation,
        other_variations: Vec<Variation>,
    ) -> Self {
        Self { id, vehicle_type, name, tags, description, difficulty, default_variation, other_variations }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn vehicle_type(&self) -> &VehicleType {
        &self.vehicle_type
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn tags(&self) -> &BTreeSet<Tag> {
        &self.tags
    }

    pub fn description(&self) -> &MarkdownText {
        &self.description
    }

    pub fn difficulty(&self) -> Difficulty {
        self.difficulty
    }

    pub fn default_variation(&self) -> &Variation {
        &self.default_variation
    }

    pub fn other_variations(&self) -> &[Variation] {
        &self.other_variations
    }

    pub fn add_tag(&mut self, tag: Tag) {
        self.tags.insert(tag);
    }

    pub fn remove_tag(&mut self, tag_id: Uuid) -> Option<Tag> {
        self.tags.take(&Tag::new(tag_id, String::new()))
    }

    pub fn update_description(&mut self, description: MarkdownText) {
        self.description = description;
    }
}
