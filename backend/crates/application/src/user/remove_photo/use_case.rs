use rc_log_domain::asset::photo_service::PhotoService;
use rc_log_domain::shared::transaction::Transaction;
use rc_log_domain::shared::unit_of_work::UnitOfWork;
use rc_log_domain::user::User;
use rc_log_domain::user::id::UserId;
use rc_log_domain::user::query::UserTransaction;
use tracing::{debug, instrument, warn};

use super::error::RemoveUserPhotoError;
use super::model::RemoveUserPhotoInput;
use crate::error::ApplicationError;

pub struct RemoveUserPhotoUseCase<UoW, PS> {
    uow: UoW,
    photo_service: PS,
}

impl<UoW, PS> RemoveUserPhotoUseCase<UoW, PS>
where
    UoW: UnitOfWork<User>,
    UoW::Transaction: UserTransaction,
    PS: PhotoService,
{
    pub fn new(uow: UoW, photo_service: PS) -> Self {
        Self { uow, photo_service }
    }

    #[instrument(skip(self, input), fields(user_id = %input.user_id))]
    pub async fn execute(&mut self, input: RemoveUserPhotoInput) -> Result<(), ApplicationError> {
        debug!("Beginning transaction");
        let mut tx = self.uow.begin().await.map_err(RemoveUserPhotoError::from)?;

        debug!("Fetching user from repository");
        let user = tx
            .get_by_id(UserId::new(input.user_id))
            .await
            .map_err(RemoveUserPhotoError::from)?
            .ok_or_else(|| {
                debug!("User not found");
                RemoveUserPhotoError::NotFound
            })?;

        let old_photo_name = user.photo_asset_name().cloned();

        let updated = User::new(
            user.id(),
            user.username().clone(),
            user.email().clone(),
            user.password_hash().clone(),
            None,
        );

        debug!("Saving updated user");
        tx.save(&updated).await.map_err(RemoveUserPhotoError::from)?;

        debug!("Committing transaction");
        tx.commit().await.map_err(RemoveUserPhotoError::from)?;

        if let Some(old_name) = old_photo_name {
            debug!("Deleting old photo (best effort)");
            if let Err(e) = self.photo_service.delete(&old_name).await {
                warn!("Failed to delete old user photo: {}", e);
            }
        }

        Ok(())
    }
}
