use rc_log_domain::model::Model;
use rc_log_domain::model::id::ModelId;
use rc_log_domain::model::transaction::ModelTransaction;
use rc_log_domain::shared::transaction::Transaction;
use rc_log_domain::shared::unit_of_work::UnitOfWork;
use tracing::{debug, instrument};
use uuid::Uuid;

use super::error::DeleteModelError;
use super::model::DeleteModelInput;
use crate::error::ApplicationError;

pub struct DeleteModelUseCase<UoW> {
    uow: UoW,
}

impl<UoW> DeleteModelUseCase<UoW>
where
    UoW: UnitOfWork<Model>,
    UoW::Transaction: ModelTransaction,
{
    pub fn new(uow: UoW) -> Self {
        Self { uow }
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

        debug!("Deleting model");
        tx.delete_by_id(ModelId::new(input.id))
            .await
            .map_err(DeleteModelError::from)?;

        debug!("Committing transaction");
        tx.commit().await.map_err(DeleteModelError::from)?;

        Ok(())
    }
}
