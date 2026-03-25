use rc_log_domain::maneuver::Maneuver;
use rc_log_domain::shared::repository::{Transaction, UnitOfWork};
use tracing::{debug, instrument};
use uuid::Uuid;

use crate::error::ApplicationError;
use super::error::GetManeuverByIdError;
use super::model::ManeuverDto;

pub struct GetManeuverByIdUseCase<UoW> {
    uow: UoW,
}

impl<UoW> GetManeuverByIdUseCase<UoW>
where
    UoW: UnitOfWork<Maneuver>,
{
    pub fn new(uow: UoW) -> Self {
        Self { uow }
    }

    #[instrument(skip(self), fields(maneuver_id = %id))]
    pub async fn execute(&mut self, id: Uuid) -> Result<ManeuverDto, ApplicationError> {
        debug!("Beginning transaction");
        let mut tx = self.uow.begin().await.map_err(GetManeuverByIdError::from)?;

        debug!("Querying maneuver from repository");
        let maneuver = tx
            .get_by_id(id)
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
