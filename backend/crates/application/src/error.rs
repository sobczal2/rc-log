use crate::maneuver::get_by_id::error::GetManeuverByIdError;
use crate::maneuver::list::error::ListManeuversError;

#[derive(Debug, thiserror::Error)]
pub enum ApplicationError {
    #[error(transparent)]
    GetManeuverById(#[from] GetManeuverByIdError),
    #[error(transparent)]
    ListManeuvers(#[from] ListManeuversError),
}
