use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum VehicleTypeDto {
    Helicopter,
    Plane,
    Drone,
}

impl std::fmt::Display for VehicleTypeDto {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VehicleTypeDto::Helicopter => write!(f, "Helicopter"),
            VehicleTypeDto::Plane => write!(f, "Plane"),
            VehicleTypeDto::Drone => write!(f, "Drone"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::VehicleTypeDto;

    #[test]
    fn serializes_as_pascalcase() {
        // Verifies the frontend contract: VehicleType = "Helicopter" | "Plane" | "Drone"
        assert_eq!(serde_json::to_string(&VehicleTypeDto::Helicopter).unwrap(), "\"Helicopter\"");
        assert_eq!(serde_json::to_string(&VehicleTypeDto::Plane).unwrap(), "\"Plane\"");
        assert_eq!(serde_json::to_string(&VehicleTypeDto::Drone).unwrap(), "\"Drone\"");
    }

    #[test]
    fn all_variants_round_trip() {
        let cases = [
            (VehicleTypeDto::Helicopter, "\"Helicopter\""),
            (VehicleTypeDto::Plane, "\"Plane\""),
            (VehicleTypeDto::Drone, "\"Drone\""),
        ];
        for (dto, expected_json) in cases {
            let serialized = serde_json::to_string(&dto).unwrap();
            assert_eq!(serialized, expected_json);
            let deserialized: VehicleTypeDto = serde_json::from_str(&serialized).unwrap();
            assert_eq!(deserialized, dto);
        }
    }
}
