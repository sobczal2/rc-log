use rc_log_domain::maneuver::Maneuver;
use rc_log_domain::shared::pagination::Pagination;
use rc_log_domain::shared::repository::{Transaction, UnitOfWork};
use tracing::{debug, instrument};

use crate::error::ApplicationError;
use crate::shared::paginated_result::PaginatedResult;

use super::error::ListManeuversError;
use super::model::ManeuverDto;

pub struct ListManeuversUseCase<UoW> {
    uow: UoW,
}

impl<UoW> ListManeuversUseCase<UoW>
where
    UoW: UnitOfWork<Maneuver>,
{
    pub fn new(uow: UoW) -> Self {
        Self { uow }
    }

    #[instrument(skip(self), fields(page = pagination.page, page_size = pagination.page_size))]
    pub async fn execute(
        &mut self,
        pagination: Pagination,
    ) -> Result<PaginatedResult<ManeuverDto>, ApplicationError> {
        debug!("Beginning transaction");
        let mut tx = self.uow.begin().await.map_err(ListManeuversError::from)?;

        debug!("Querying maneuvers from repository");
        let (maneuvers, total) = tx.list(pagination).await.map_err(ListManeuversError::from)?;

        debug!(count = maneuvers.len(), total, "Maneuvers retrieved, committing transaction");
        tx.commit().await.map_err(ListManeuversError::from)?;

        let dtos = maneuvers.into_iter().map(ManeuverDto::from).collect();
        Ok(PaginatedResult::new(dtos, total, pagination.page, pagination.page_size))
    }
}
