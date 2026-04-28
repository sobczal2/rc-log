use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum SortDirectionDto {
    Asc,
    Desc,
}

impl Default for SortDirectionDto {
    fn default() -> Self {
        SortDirectionDto::Asc
    }
}
