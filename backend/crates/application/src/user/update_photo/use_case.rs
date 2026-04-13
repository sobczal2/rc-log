use rc_log_domain::asset::name::AssetName;
use rc_log_domain::asset::photo_storage::PhotoStorage;
use rc_log_domain::shared::transaction::Transaction;
use rc_log_domain::shared::unit_of_work::UnitOfWork;
use rc_log_domain::user::User;
use rc_log_domain::user::id::UserId;
use rc_log_domain::user::query::UserTransaction;
use tracing::{debug, instrument, warn};
use uuid::Uuid;

use super::error::UpdateUserPhotoError;
use super::model::{UpdateUserPhotoInput, UserDto};
use crate::error::ApplicationError;

pub struct UpdateUserPhotoUseCase<UoW, PS> {
    uow: UoW,
    photo_storage: PS,
}

impl<UoW, PS> UpdateUserPhotoUseCase<UoW, PS>
where
    UoW: UnitOfWork<User>,
    UoW::Transaction: UserTransaction,
    PS: PhotoStorage,
{
    pub fn new(uow: UoW, photo_storage: PS) -> Self {
        Self { uow, photo_storage }
    }

    #[instrument(skip(self, input), fields(user_id = %input.user_id))]
    pub async fn execute(
        &mut self,
        input: UpdateUserPhotoInput,
    ) -> Result<UserDto, ApplicationError> {
        debug!("Beginning transaction");
        let mut tx = self.uow.begin().await.map_err(UpdateUserPhotoError::from)?;

        debug!("Fetching user from repository");
        let user = tx
            .get_by_id(UserId::new(input.user_id))
            .await
            .map_err(UpdateUserPhotoError::from)?
            .ok_or_else(|| {
                debug!("User not found");
                UpdateUserPhotoError::NotFound
            })?;

        let old_photo_name = user.photo_asset_name().cloned();

        let new_asset_name = AssetName::new(format!("user-photo-{}", Uuid::new_v4()))
            .map_err(|e| UpdateUserPhotoError::InvalidData(e.to_string()))?;

        debug!("Storing new photo");
        let new_photo = self
            .photo_storage
            .store(&new_asset_name, &input.data)
            .await
            .map_err(UpdateUserPhotoError::from)?;

        let updated = User::new(
            user.id(),
            user.username().clone(),
            user.email().clone(),
            user.password_hash().clone(),
            Some(new_photo.name.clone()),
        );

        debug!("Saving updated user");
        tx.save(&updated).await.map_err(UpdateUserPhotoError::from)?;

        debug!("Committing transaction");
        tx.commit().await.map_err(UpdateUserPhotoError::from)?;

        if let Some(old_name) = old_photo_name {
            debug!("Deleting old photo (best effort)");
            if let Err(e) = self.photo_storage.delete(&old_name).await {
                warn!("Failed to delete old user photo: {}", e);
            }
        }

        Ok(UserDto::from(updated))
    }
}
