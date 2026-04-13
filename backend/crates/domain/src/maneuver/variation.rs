use uuid::Uuid;

use crate::asset::name::AssetName;
use crate::maneuver::difficulty::Difficulty;
use crate::shared::markdown_text::MarkdownText;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VariationId(Uuid);

impl VariationId {
    pub fn new(uuid: Uuid) -> Self {
        Self(uuid)
    }

    pub fn as_uuid(&self) -> Uuid {
        self.0
    }
}

impl From<Uuid> for VariationId {
    fn from(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

impl From<VariationId> for Uuid {
    fn from(id: VariationId) -> Uuid {
        id.0
    }
}

#[derive(Debug, Clone)]
pub struct Variation {
    id: VariationId,
    name: String,
    description: MarkdownText,
    video_asset_name: AssetName,
    difficulty: Difficulty,
}

impl Variation {
    pub fn new(
        id: VariationId,
        name: String,
        description: MarkdownText,
        video_asset_name: AssetName,
        difficulty: Difficulty,
    ) -> Self {
        Self { id, name, description, video_asset_name, difficulty }
    }

    pub fn id(&self) -> VariationId {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> &MarkdownText {
        &self.description
    }

    pub fn video_asset_name(&self) -> &AssetName {
        &self.video_asset_name
    }

    pub fn difficulty(&self) -> Difficulty {
        self.difficulty
    }
}
