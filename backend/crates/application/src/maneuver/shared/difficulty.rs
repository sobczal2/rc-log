use serde::{Deserialize, Serialize};
use specta::Type as SpectaType;

#[derive(Debug, Clone, Serialize, Deserialize, SpectaType, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum DifficultyDto {
    Level1,
    Level2,
    Level3,
    Level4,
    Level5,
    Level6,
    Level7,
}
