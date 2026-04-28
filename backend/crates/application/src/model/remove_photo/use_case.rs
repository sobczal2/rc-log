use rc_log_domain::asset::photo_service::PhotoService;
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

pub struct RemoveModelPhotoUseCase<UoW, PS> {
    uow: UoW,
    photo_service: PS,
}

impl<UoW, PS> RemoveModelPhotoUseCase<UoW, PS>
where
    UoW: UnitOfWork<Model>,
    UoW::Transaction: ModelTransaction,
    PS: PhotoService,
{
    pub fn new(uow: UoW, photo_service: PS) -> Self {
        Self { uow, photo_service }
    }

    #[instrument(skip(self, input), fields(model_id = %input.model_id, owner_id = %input.owner_id))]
    pub async fn execute(&mut self, input: RemoveModelPhotoInput) -> Result<(), RemoveModelPhotoError> {
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
            debug!("Model belongs to a different owner, returning NotFound");
            tx.rollback().await.map_err(RemoveModelPhotoError::from)?;
            return Err(RemoveModelPhotoError::NotFound);
        }

        let old_photo_id = model.photo_asset_id().cloned();

        let updated = Model::new(
            model.id(),
            UserId::new(input.owner_id),
            model.name().clone(),
            model.r#type(),
            None,
        );

        debug!("Saving updated model with photo cleared");
        tx.save(&updated).await.map_err(RemoveModelPhotoError::from)?;

        debug!("Committing transaction");
        tx.commit().await.map_err(RemoveModelPhotoError::from)?;

        if let Some(old_photo_id) = old_photo_id {
            debug!(photo_id = %old_photo_id.as_uuid(), "Deleting photo from storage (best-effort)");
            if let Err(e) = self.photo_service.delete(&old_photo_id).await {
                warn!(error = %e, photo_id = %old_photo_id.as_uuid(), "Failed to delete photo");
            }
        }

        Ok(())
    }
}
