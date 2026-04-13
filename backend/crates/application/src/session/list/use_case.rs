use rc_log_domain::session::Session;
use rc_log_domain::session::transaction::SessionTransaction;
use rc_log_domain::shared::pagination::Pagination;
use rc_log_domain::shared::transaction::Transaction;
use rc_log_domain::shared::unit_of_work::UnitOfWork;
use rc_log_domain::user::id::UserId;
use tracing::{debug, instrument};

use super::error::ListSessionsError;
use super::model::{ListSessionsInput, SessionDto};
use crate::error::ApplicationError;
use crate::shared::pagination::PaginatedResult;

pub struct ListSessionsUseCase<UoW> {
    uow: UoW,
}

impl<UoW> ListSessionsUseCase<UoW>
where
    UoW: UnitOfWork<Session>,
    UoW::Transaction: SessionTransaction,
{
    pub fn new(uow: UoW) -> Self {
        Self { uow }
    }

    #[instrument(skip(self), fields(owner_id = %input.owner_id, page = input.pagination.page, page_size = input.pagination.page_size))]
    pub async fn execute(
        &mut self,
        input: ListSessionsInput,
    ) -> Result<PaginatedResult<SessionDto>, ApplicationError> {
        debug!("Beginning transaction");
        let mut tx = self.uow.begin().await.map_err(ListSessionsError::from)?;

        let page = input.pagination.page;
        let page_size = input.pagination.page_size;
        let owner_id = UserId::new(input.owner_id);
        let pagination = Pagination::from(input.pagination);
        let filter = input.filter.into();
        let sort = input.sort.into();

        debug!("Querying sessions from repository");
        let (sessions, total) = tx
            .list_by_owner(owner_id, pagination, filter, sort)
            .await
            .map_err(ListSessionsError::from)?;

        debug!(count = sessions.len(), total, "Sessions retrieved, committing transaction");
        tx.commit().await.map_err(ListSessionsError::from)?;

        let items = sessions.into_iter().map(SessionDto::from).collect();
        Ok(PaginatedResult::new(items, total, page, page_size))
    }
}
