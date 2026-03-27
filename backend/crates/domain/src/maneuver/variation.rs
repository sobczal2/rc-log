use uuid::Uuid;

use crate::asset::name::AssetName;
use crate::shared::markdown_text::MarkdownText;

#[derive(Debug, Clone)]
pub struct Variation {
    id: Uuid,
    name: String,
    description: MarkdownText,
    video_asset_name: AssetName,
}

impl Variation {
    pub fn new(id: Uuid, name: String, description: MarkdownText, video_asset_name: AssetName) -> Self {
        Self { id, name, description, video_asset_name }
    }

    pub fn id(&self) -> Uuid {
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
}
