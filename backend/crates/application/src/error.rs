use crate::maneuver::error::ManeuverError;

#[derive(Debug, thiserror::Error)]
pub enum ApplicationError {
    #[error(transparent)]
    Maneuver(#[from] ManeuverError),
}
