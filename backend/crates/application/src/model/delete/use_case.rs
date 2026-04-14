use rc_log_domain::asset::photo_service::PhotoService;
use rc_log_domain::model::Model;
use rc_log_domain::model::id::ModelId;
use rc_log_domain::model::transaction::ModelTransaction;
use rc_log_domain::shared::transaction::Transaction;
use rc_log_domain::shared::unit_of_work::UnitOfWork;
use tracing::{debug, instrument, warn};
use uuid::Uuid;

use super::error::DeleteModelError;
use super::model::DeleteModelInput;
use crate::error::ApplicationError;

pub struct DeleteModelUseCase<UoW, PS> {
    uow: UoW,
    photo_service: PS,
}

impl<UoW, PS> DeleteModelUseCase<UoW, PS>
where
    UoW: UnitOfWork<Model>,
    UoW::Transaction: ModelTransaction,
    PS: PhotoService,
{
    pub fn new(uow: UoW, photo_service: PS) -> Self {
        Self { uow, photo_service }
    }

    #[instrument(skip(self), fields(model_id = %input.id, owner_id = %input.owner_id))]
    pub async fn execute(&mut self, input: DeleteModelInput) -> Result<(), ApplicationError> {
        debug!("Beginning transaction");
        let mut tx = self.uow.begin().await.map_err(DeleteModelError::from)?;

        debug!("Checking model exists and verifying ownership");
        let model = tx
            .get_by_id(ModelId::new(input.id))
            .await
            .map_err(DeleteModelError::from)?
            .ok_or_else(|| {
                debug!("Model not found");
                DeleteModelError::NotFound
            })?;

        if Uuid::from(model.owner_id()) != input.owner_id {
            debug!("Model belongs to a different owner, returning Forbidden");
            tx.rollback().await.map_err(DeleteModelError::from)?;
            return Err(DeleteModelError::Forbidden.into());
        }

        let photo_id = model.photo_asset_id().cloned();

        debug!("Deleting model");
        tx.delete_by_id(ModelId::new(input.id)).await.map_err(DeleteModelError::from)?;

        debug!("Committing transaction");
        tx.commit().await.map_err(DeleteModelError::from)?;

        if let Some(photo_id) = photo_id {
            debug!(photo_id = %photo_id.as_uuid(), "Deleting model photo (best-effort)");
            if let Err(e) = self.photo_service.delete(&photo_id).await {
                warn!(error = %e, photo_id = %photo_id.as_uuid(), "Failed to delete model photo");
            }
        }

        Ok(())
    }
}
