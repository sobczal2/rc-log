use crate::maneuver::get_by_id::error::GetManeuverByIdError;
use crate::maneuver::list::error::ListManeuversError;
use crate::model::create::error::CreateModelError;
use crate::model::delete::error::DeleteModelError;
use crate::model::get_by_id::error::GetModelByIdError;
use crate::model::list::error::ListModelsError;
use crate::model::remove_photo::error::RemoveModelPhotoError;
use crate::model::update::error::UpdateModelError;
use crate::model::update_photo::error::UpdateModelPhotoError;
use crate::photo::resolve::error::ResolvePhotoError;
use crate::session::add_performed_variation::error::AddPerformedVariationError;
use crate::session::create::error::CreateSessionError;
use crate::session::list::error::ListSessionsError;
use crate::session::remove_performed_variation::error::RemovePerformedVariationError;
use crate::session::update::error::UpdateSessionError;
use crate::user::get_by_id::error::GetUserByIdError;
use crate::user::get_by_username::error::GetUserByUsernameError;
use crate::user::remove_photo::error::RemoveUserPhotoError;
use crate::user::sign_in::error::SignInError;
use crate::user::sign_up::error::SignUpError;
use crate::user::update::error::UpdateUserError;
use crate::user::update_photo::error::UpdateUserPhotoError;
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
    UpdateModelPhoto(#[from] UpdateModelPhotoError),
    #[error(transparent)]
    RemoveModelPhoto(#[from] RemoveModelPhotoError),
    #[error(transparent)]
    CreateSession(#[from] CreateSessionError),
    #[error(transparent)]
    ListSessions(#[from] ListSessionsError),
    #[error(transparent)]
    AddPerformedVariation(#[from] AddPerformedVariationError),
    #[error(transparent)]
    RemovePerformedVariation(#[from] RemovePerformedVariationError),
    #[error(transparent)]
    UpdateSession(#[from] UpdateSessionError),
    #[error(transparent)]
    GetUserById(#[from] GetUserByIdError),
    #[error(transparent)]
    GetUserByUsername(#[from] GetUserByUsernameError),
    #[error(transparent)]
    SignIn(#[from] SignInError),
    #[error(transparent)]
    SignUp(#[from] SignUpError),
    #[error(transparent)]
    UpdateUser(#[from] UpdateUserError),
    #[error(transparent)]
    UpdateUserPhoto(#[from] UpdateUserPhotoError),
    #[error(transparent)]
    RemoveUserPhoto(#[from] RemoveUserPhotoError),
    #[error(transparent)]
    ResolveVideo(#[from] ResolveVideoError),
    #[error(transparent)]
    ResolvePhoto(#[from] ResolvePhotoError),
}
