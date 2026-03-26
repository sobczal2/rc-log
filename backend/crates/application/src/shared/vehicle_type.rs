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
