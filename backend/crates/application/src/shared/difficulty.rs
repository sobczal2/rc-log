use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
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
