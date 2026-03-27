use crate::maneuver::get_by_id::error::GetManeuverByIdError;
use crate::maneuver::list::error::ListManeuversError;
use crate::user::create::error::CreateUserError;
use crate::user::get_by_id::error::GetUserByIdError;
use crate::user::get_by_username::error::GetUserByUsernameError;
use crate::user::sign_in::error::SignInError;
use crate::user::sign_up::error::SignUpError;

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
    #[error(transparent)]
    SignIn(#[from] SignInError),
    #[error(transparent)]
    SignUp(#[from] SignUpError),
}
