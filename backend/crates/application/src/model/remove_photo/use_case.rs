use rc_log_domain::asset::photo_storage::PhotoStorage;
use rc_log_domain::model::Model;
use rc_log_domain::model::id::ModelId;
use rc_log_domain::model::transaction::ModelTransaction;
use rc_log_domain::shared::transaction::Transaction;
use rc_log_domain::shared::unit_of_work::UnitOfWork;
use rc_log_domain::user::id::UserId;
use tracing::{debug, instrument, warn};
use uuid::Uuid;

use super::error::RemoveModelPhotoError;
use super::model::RemoveModelPhotoInput;
use crate::error::ApplicationError;

pub struct RemoveModelPhotoUseCase<UoW, PS> {
    uow: UoW,
    photo_storage: PS,
}

impl<UoW, PS> RemoveModelPhotoUseCase<UoW, PS>
where
    UoW: UnitOfWork<Model>,
    UoW::Transaction: ModelTransaction,
    PS: PhotoStorage,
{
    pub fn new(uow: UoW, photo_storage: PS) -> Self {
        Self { uow, photo_storage }
    }

    #[instrument(skip(self, input), fields(model_id = %input.model_id, owner_id = %input.owner_id))]
    pub async fn execute(&mut self, input: RemoveModelPhotoInput) -> Result<(), ApplicationError> {
        debug!("Beginning transaction");
        let mut tx = self.uow.begin().await.map_err(RemoveModelPhotoError::from)?;

        debug!("Fetching model and verifying ownership");
        let model = tx
            .get_by_id(ModelId::new(input.model_id))
            .await
            .map_err(RemoveModelPhotoError::from)?
            .ok_or_else(|| {
                debug!("Model not found");
                RemoveModelPhotoError::NotFound
            })?;

        if Uuid::from(model.owner_id()) != input.owner_id {
            debug!("Model belongs to a different owner, returning Forbidden");
            tx.rollback().await.map_err(RemoveModelPhotoError::from)?;
            return Err(RemoveModelPhotoError::Forbidden.into());
        }

        let old_photo_name = model.photo_asset_name().cloned();

        let updated = Model::new(
            model.id(),
            UserId::new(input.owner_id),
            model.name().clone(),
            model.vehicle_type(),
            None,
        );

        debug!("Saving updated model with photo cleared");
        tx.save(&updated).await.map_err(RemoveModelPhotoError::from)?;

        debug!("Committing transaction");
        tx.commit().await.map_err(RemoveModelPhotoError::from)?;

        if let Some(old_name) = old_photo_name {
            debug!(name = %old_name.as_str(), "Deleting photo from storage (best-effort)");
            if let Err(e) = self.photo_storage.delete(&old_name).await {
                warn!(error = %e, name = %old_name.as_str(), "Failed to delete photo");
            }
        }

        Ok(())
    }
}
