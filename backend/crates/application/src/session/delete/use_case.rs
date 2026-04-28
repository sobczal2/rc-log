use rc_log_domain::session::Session;
use rc_log_domain::session::id::SessionId;
use rc_log_domain::session::transaction::SessionTransaction;
use rc_log_domain::shared::transaction::Transaction;
use rc_log_domain::shared::unit_of_work::UnitOfWork;
use tracing::{debug, instrument};
use uuid::Uuid;

use super::error::DeleteSessionError;
use super::model::DeleteSessionInput;

pub struct DeleteSessionUseCase<UoW> {
    uow: UoW,
}

impl<UoW> DeleteSessionUseCase<UoW>
where
    UoW: UnitOfWork<Session>,
    UoW::Transaction: SessionTransaction,
{
    pub fn new(uow: UoW) -> Self {
        Self { uow }
    }

    #[instrument(skip(self), fields(session_id = %input.id, owner_id = %input.owner_id))]
    pub async fn execute(&mut self, input: DeleteSessionInput) -> Result<(), DeleteSessionError> {
        debug!("Beginning transaction");
        let mut tx = self.uow.begin().await.map_err(DeleteSessionError::from)?;

        debug!("Checking session exists and verifying ownership");
        let session = tx
            .get_by_id(SessionId::new(input.id))
            .await
            .map_err(DeleteSessionError::from)?
            .ok_or_else(|| {
                debug!("Session not found");
                DeleteSessionError::NotFound
            })?;

        if Uuid::from(session.user_id()) != input.owner_id {
            debug!("Session belongs to a different owner, returning Forbidden");
            tx.rollback().await.map_err(DeleteSessionError::from)?;
            return Err(DeleteSessionError::Forbidden);
        }

        debug!("Deleting session");
        tx.delete_by_id(SessionId::new(input.id)).await.map_err(DeleteSessionError::from)?;

        debug!("Committing transaction");
        tx.commit().await.map_err(DeleteSessionError::from)?;

        Ok(())
    }
}
