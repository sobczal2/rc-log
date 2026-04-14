use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum TypeDto {
    Helicopter,
    Plane,
    Drone,
}

impl std::fmt::Display for TypeDto {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TypeDto::Helicopter => write!(f, "Helicopter"),
            TypeDto::Plane => write!(f, "Plane"),
            TypeDto::Drone => write!(f, "Drone"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::TypeDto;

    #[test]
    fn serializes_as_pascal_case_labels_expected_by_frontend() {
        assert_eq!(serde_json::to_string(&TypeDto::Helicopter).unwrap(), "\"Helicopter\"");
        assert_eq!(serde_json::to_string(&TypeDto::Plane).unwrap(), "\"Plane\"");
        assert_eq!(serde_json::to_string(&TypeDto::Drone).unwrap(), "\"Drone\"");
    }

    #[test]
    fn round_trip_serialization_deserialization() {
        let cases = [
            (TypeDto::Helicopter, "\"Helicopter\""),
            (TypeDto::Plane, "\"Plane\""),
            (TypeDto::Drone, "\"Drone\""),
        ];

        for (value, serialized) in cases {
            assert_eq!(serde_json::to_string(&value).unwrap(), serialized);
            let deserialized: TypeDto = serde_json::from_str(serialized).unwrap();
            assert_eq!(deserialized, value);
        }
    }
}
