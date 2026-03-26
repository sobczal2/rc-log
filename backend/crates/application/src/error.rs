use crate::maneuver::get_by_id::error::GetManeuverByIdError;
use crate::maneuver::list::error::ListManeuversError;
use crate::user::create::error::CreateUserError;
use crate::user::get_by_id::error::GetUserByIdError;
use crate::user::get_by_username::error::GetUserByUsernameError;

#[derive(Debug, thiserror::Error)]
pub enum ApplicationError {
    #[error(transparent)]
    GetManeuverById(#[from] GetManeuverByIdError),
    #[error(transparent)]
    ListManeuvers(#[from] ListManeuversError),
    #[error(transparent)]
    CreateUser(#[from] CreateUserError),
    #[error(transparent)]
    GetUserById(#[from] GetUserByIdError),
    #[error(transparent)]
    GetUserByUsername(#[from] GetUserByUsernameError),
}
