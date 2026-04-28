use rc_log_domain::session::Session;
use rc_log_domain::session::id::SessionId;
use rc_log_domain::session::transaction::SessionTransaction;
use rc_log_domain::shared::transaction::Transaction;
use rc_log_domain::shared::unit_of_work::UnitOfWork;
use tracing::{debug, instrument};
use uuid::Uuid;

use super::error::RemovePerformedVariationError;
use super::model::RemovePerformedVariationInput;

pub struct RemovePerformedVariationUseCase<UoW> {
    uow: UoW,
}

impl<UoW> RemovePerformedVariationUseCase<UoW>
where
    UoW: UnitOfWork<Session>,
    UoW::Transaction: SessionTransaction,
{
    pub fn new(uow: UoW) -> Self {
        Self { uow }
    }

    #[instrument(skip(self), fields(session_id = %input.session_id, owner_id = %input.owner_id, performed_variation_id = %input.performed_variation_id))]
    pub async fn execute(
        &mut self,
        input: RemovePerformedVariationInput,
    ) -> Result<(), RemovePerformedVariationError> {
        debug!("Beginning transaction");
        let mut tx = self.uow.begin().await.map_err(RemovePerformedVariationError::from)?;

        let existing = tx
            .get_by_id(SessionId::new(input.session_id))
            .await
            .map_err(RemovePerformedVariationError::from)?
            .ok_or(RemovePerformedVariationError::NotFound)?;

        if Uuid::from(existing.user_id()) != input.owner_id {
            tx.rollback().await.map_err(RemovePerformedVariationError::from)?;
            return Err(RemovePerformedVariationError::Forbidden);
        }

        let before_count = existing.performed_variations().len();
        let mut performed_variations = existing.performed_variations().to_vec();
        performed_variations.retain(|pv| Uuid::from(pv.id()) != input.performed_variation_id);
        performed_variations.sort_by_key(|pv| pv.id().as_uuid());

        if performed_variations.len() == before_count {
            tx.rollback().await.map_err(RemovePerformedVariationError::from)?;
            return Err(RemovePerformedVariationError::PerformedVariationNotFound);
        }

        let updated = Session::new(
            existing.id(),
            existing.user_id(),
            existing.date().clone(),
            existing.model_id(),
            existing.note().cloned(),
            performed_variations,
        );

        tx.save(&updated).await.map_err(RemovePerformedVariationError::from)?;
        tx.commit().await.map_err(RemovePerformedVariationError::from)?;

        Ok(())
    }
}
