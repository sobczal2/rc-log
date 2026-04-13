use rc_log_domain::model::Model;
use rc_log_domain::model::transaction::ModelTransaction;
use rc_log_domain::shared::pagination::Pagination;
use rc_log_domain::shared::transaction::Transaction;
use rc_log_domain::shared::unit_of_work::UnitOfWork;
use rc_log_domain::user::id::UserId;
use tracing::{debug, instrument};

use super::error::ListModelsError;
use super::model::{ListModelsInput, ModelDto};
use crate::error::ApplicationError;
use crate::shared::pagination::PaginatedResult;

pub struct ListModelsUseCase<UoW> {
    uow: UoW,
}

impl<UoW> ListModelsUseCase<UoW>
where
    UoW: UnitOfWork<Model>,
    UoW::Transaction: ModelTransaction,
{
    pub fn new(uow: UoW) -> Self {
        Self { uow }
    }

    #[instrument(skip(self), fields(owner_id = %input.owner_id, page = input.pagination.page, page_size = input.pagination.page_size))]
    pub async fn execute(
        &mut self,
        input: ListModelsInput,
    ) -> Result<PaginatedResult<ModelDto>, ApplicationError> {
        debug!("Beginning transaction");
        let mut tx = self.uow.begin().await.map_err(ListModelsError::from)?;

        let page = input.pagination.page;
        let page_size = input.pagination.page_size;
        let pagination = Pagination::from(input.pagination);
        let owner_id = UserId::new(input.owner_id);

        debug!("Querying models from repository");
        let (models, total) =
            tx.list_by_owner(owner_id, pagination).await.map_err(ListModelsError::from)?;

        debug!(total, count = models.len(), "Models retrieved, committing transaction");
        tx.commit().await.map_err(ListModelsError::from)?;

        let items = models.into_iter().map(ModelDto::from).collect();
        Ok(PaginatedResult::new(items, total, page, page_size))
    }
}
