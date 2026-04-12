use crate::maneuver::get_by_id::error::GetManeuverByIdError;
use crate::maneuver::list::error::ListManeuversError;
use crate::model::create::error::CreateModelError;
use crate::model::delete::error::DeleteModelError;
use crate::model::get_by_id::error::GetModelByIdError;
use crate::model::list::error::ListModelsError;
use crate::model::update::error::UpdateModelError;
use crate::photo::resolve::error::ResolvePhotoError;
use crate::user::get_by_id::error::GetUserByIdError;
use crate::user::get_by_username::error::GetUserByUsernameError;
use crate::user::sign_in::error::SignInError;
use crate::user::sign_up::error::SignUpError;
use crate::video::resolve::error::ResolveVideoError;

#[derive(Debug, thiserror::Error)]
pub enum ApplicationError {
    #[error(transparent)]
    GetManeuverById(#[from] GetManeuverByIdError),
    #[error(transparent)]
    ListManeuvers(#[from] ListManeuversError),
    #[error(transparent)]
    GetModelById(#[from] GetModelByIdError),
    #[error(transparent)]
    ListModels(#[from] ListModelsError),
    #[error(transparent)]
    CreateModel(#[from] CreateModelError),
    #[error(transparent)]
    UpdateModel(#[from] UpdateModelError),
    #[error(transparent)]
    DeleteModel(#[from] DeleteModelError),
    #[error(transparent)]
    GetUserById(#[from] GetUserByIdError),
    #[error(transparent)]
    GetUserByUsername(#[from] GetUserByUsernameError),
    #[error(transparent)]
    SignIn(#[from] SignInError),
    #[error(transparent)]
    SignUp(#[from] SignUpError),
    #[error(transparent)]
    ResolveVideo(#[from] ResolveVideoError),
    #[error(transparent)]
    ResolvePhoto(#[from] ResolvePhotoError),
}
