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

#[cfg(test)]
mod tests {
    use super::DifficultyDto;

    #[test]
    fn serializes_as_camelcase_lowercase() {
        // Verifies the frontend contract: DifficultyLevel = "level1" | ... | "level7"
        assert_eq!(serde_json::to_string(&DifficultyDto::Level1).unwrap(), "\"level1\"");
        assert_eq!(serde_json::to_string(&DifficultyDto::Level7).unwrap(), "\"level7\"");
    }

    #[test]
    fn all_levels_round_trip() {
        let cases = [
            (DifficultyDto::Level1, "\"level1\""),
            (DifficultyDto::Level2, "\"level2\""),
            (DifficultyDto::Level3, "\"level3\""),
            (DifficultyDto::Level4, "\"level4\""),
            (DifficultyDto::Level5, "\"level5\""),
            (DifficultyDto::Level6, "\"level6\""),
            (DifficultyDto::Level7, "\"level7\""),
        ];
        for (dto, expected_json) in cases {
            let serialized = serde_json::to_string(&dto).unwrap();
            assert_eq!(serialized, expected_json);
            let deserialized: DifficultyDto = serde_json::from_str(&serialized).unwrap();
            assert_eq!(deserialized, dto);
        }
    }
}
