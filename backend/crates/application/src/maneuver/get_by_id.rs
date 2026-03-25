use rc_log_domain::maneuver::Maneuver;
use rc_log_domain::shared::repository::{Transaction, UnitOfWork};
use uuid::Uuid;

use crate::error::ApplicationError;
use crate::maneuver::error::ManeuverError;

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

    pub async fn execute(&mut self, id: Uuid) -> Result<Maneuver, ApplicationError> {
        let mut tx = self.uow.begin().await.map_err(ManeuverError::from)?;

        let maneuver =
            tx.get_by_id(id).await.map_err(ManeuverError::from)?.ok_or(ManeuverError::NotFound)?;

        tx.commit().await.map_err(ManeuverError::from)?;

        Ok(maneuver)
    }
}
