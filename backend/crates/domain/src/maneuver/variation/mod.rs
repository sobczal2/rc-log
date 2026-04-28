pub mod id;
pub mod resolver;

pub use id::VariationId;

use crate::video::VideoId;
use crate::maneuver::difficulty::Difficulty;
use crate::maneuver::id::ManeuverId;
use crate::shared::markdown_text::MarkdownText;

#[derive(Debug, Clone)]
pub struct Variation {
    id: VariationId,
    maneuver_id: ManeuverId,
    name: String,
    description: MarkdownText,
    video_asset_id: VideoId,
    difficulty: Difficulty,
}

impl Variation {
    pub fn new(
        id: VariationId,
        maneuver_id: ManeuverId,
        name: String,
        description: MarkdownText,
        video_asset_id: VideoId,
        difficulty: Difficulty,
    ) -> Self {
        Self { id, maneuver_id, name, description, video_asset_id, difficulty }
    }

    pub fn id(&self) -> VariationId {
        self.id
    }

    pub fn maneuver_id(&self) -> ManeuverId {
        self.maneuver_id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> &MarkdownText {
        &self.description
    }

    pub fn video_asset_id(&self) -> VideoId {
        self.video_asset_id
    }

    pub fn difficulty(&self) -> Difficulty {
        self.difficulty
    }
}
