use rc_log_domain::maneuver::Maneuver;
use rc_log_domain::maneuver::id::ManeuverId;
use rc_log_domain::maneuver::transaction::ManeuverTransaction;
use rc_log_domain::shared::transaction::Transaction;
use rc_log_domain::shared::unit_of_work::UnitOfWork;
use tracing::{debug, instrument};

use super::error::GetManeuverByIdError;
use super::model::{GetManeuverByIdInput, ManeuverDto};
use crate::error::ApplicationError;

pub struct GetManeuverByIdUseCase<UoW> {
    uow: UoW,
}

impl<UoW> GetManeuverByIdUseCase<UoW>
where
    UoW: UnitOfWork<Maneuver>,
    UoW::Transaction: ManeuverTransaction,
{
    pub fn new(uow: UoW) -> Self {
        Self { uow }
    }

    #[instrument(skip(self), fields(maneuver_id = %input.id))]
    pub async fn execute(
        &mut self,
        input: GetManeuverByIdInput,
    ) -> Result<ManeuverDto, ApplicationError> {
        debug!("Beginning transaction");
        let mut tx = self.uow.begin().await.map_err(GetManeuverByIdError::from)?;

        debug!("Querying maneuver from repository");
        let maneuver = tx
            .get_by_id(ManeuverId::new(input.id))
            .await
            .map_err(GetManeuverByIdError::from)?
            .ok_or_else(|| {
                debug!("Maneuver not found in repository");
                GetManeuverByIdError::NotFound
            })?;

        debug!(name = maneuver.name(), "Maneuver retrieved, committing transaction");
        tx.commit().await.map_err(GetManeuverByIdError::from)?;

        Ok(ManeuverDto::from(maneuver))
    }
}
