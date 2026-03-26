pub mod difficulty;
pub mod query;
pub mod tag;

use std::collections::BTreeSet;

use uuid::Uuid;

use crate::{
    maneuver::{difficulty::Difficulty, tag::Tag},
    shared::{markdown_text::MarkdownText, vehicle_type::VehicleType, video_path::VideoPath},
};

#[derive(Debug, Clone)]
pub struct Maneuver {
    id: Uuid,
    vehicle_type: VehicleType,
    name: String,
    tags: BTreeSet<Tag>,
    description: MarkdownText,
    difficulty: Difficulty,
    video_path: Option<VideoPath>,
}

impl Maneuver {
    pub fn new(
        id: Uuid,
        vehicle_type: VehicleType,
        name: String,
        tags: BTreeSet<Tag>,
        description: MarkdownText,
        difficulty: Difficulty,
        video_path: Option<VideoPath>,
    ) -> Self {
        Self { id, vehicle_type, name, tags, description, difficulty, video_path }
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

    pub fn video_path(&self) -> Option<&VideoPath> {
        self.video_path.as_ref()
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

    pub fn update_video_path(&mut self, video_path: Option<VideoPath>) {
        self.video_path = video_path;
    }
}
