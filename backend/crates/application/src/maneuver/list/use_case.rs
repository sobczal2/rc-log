use rc_log_domain::maneuver::Maneuver;
use rc_log_domain::maneuver::transaction::ManeuverTransaction;
use rc_log_domain::shared::pagination::Pagination;
use rc_log_domain::shared::transaction::Transaction;
use rc_log_domain::shared::unit_of_work::UnitOfWork;
use tracing::{debug, instrument};

use crate::error::ApplicationError;
use crate::shared::pagination::PaginatedResult;

use super::error::ListManeuversError;
use super::model::{ListManeuversInput, ManeuverDto};
use rc_log_domain::maneuver::transaction::{ManeuverFilter, ManeuverSort};

pub struct ListManeuversUseCase<UoW> {
    uow: UoW,
}

impl<UoW> ListManeuversUseCase<UoW>
where
    UoW: UnitOfWork<Maneuver>,
    UoW::Transaction: ManeuverTransaction,
{
    pub fn new(uow: UoW) -> Self {
        Self { uow }
    }

    #[instrument(skip(self), fields(page = input.pagination.page, page_size = input.pagination.page_size))]
    pub async fn execute(
        &mut self,
        input: ListManeuversInput,
    ) -> Result<PaginatedResult<ManeuverDto>, ApplicationError> {
        debug!("Beginning transaction");
        let mut tx = self.uow.begin().await.map_err(ListManeuversError::from)?;

        debug!("Querying maneuvers from repository");
        let page = input.pagination.page;
        let page_size = input.pagination.page_size;
        let domain_pagination = Pagination::from(input.pagination);
        let domain_filter = ManeuverFilter::from(input.filter);
        let domain_sort = ManeuverSort::from(input.sort);

        let (maneuvers, total) = tx
            .list(domain_pagination, domain_filter, domain_sort)
            .await
            .map_err(ListManeuversError::from)?;

        debug!(count = maneuvers.len(), total, "Maneuvers retrieved, committing transaction");
        tx.commit().await.map_err(ListManeuversError::from)?;

        let dtos = maneuvers.into_iter().map(ManeuverDto::from).collect();
        Ok(PaginatedResult::new(dtos, total, page, page_size))
    }
}
