use rc_log_domain::model::Model;
use rc_log_domain::model::Type;
use rc_log_domain::model::id::ModelId;
use rc_log_domain::model::name::Name;
use rc_log_domain::model::transaction::ModelTransaction;
use rc_log_domain::shared::transaction::Transaction;
use rc_log_domain::shared::unit_of_work::UnitOfWork;
use rc_log_domain::user::id::UserId;
use tracing::{debug, instrument};
use uuid::Uuid;

use super::error::UpdateModelError;
use super::model::{ModelDto, UpdateModelInput};
use crate::error::ApplicationError;
use crate::model::shared::TypeDto;

pub struct UpdateModelUseCase<UoW> {
    uow: UoW,
}

impl<UoW> UpdateModelUseCase<UoW>
where
    UoW: UnitOfWork<Model>,
    UoW::Transaction: ModelTransaction,
{
    pub fn new(uow: UoW) -> Self {
        Self { uow }
    }

    #[instrument(skip(self), fields(model_id = %input.id, owner_id = %input.owner_id))]
    pub async fn execute(&mut self, input: UpdateModelInput) -> Result<ModelDto, ApplicationError> {
        let name =
            Name::new(input.name).map_err(|e| UpdateModelError::ValidationError(e.to_string()))?;

        let r#type = match input.r#type {
            TypeDto::Helicopter => Type::Helicopter,
            TypeDto::Plane => Type::Plane,
            TypeDto::Drone => Type::Drone,
        };

        debug!("Beginning transaction");
        let mut tx = self.uow.begin().await.map_err(UpdateModelError::from)?;

        debug!("Checking model exists and verifying ownership");
        let existing = tx
            .get_by_id(ModelId::new(input.id))
            .await
            .map_err(UpdateModelError::from)?
            .ok_or_else(|| {
                debug!("Model not found");
                UpdateModelError::NotFound
            })?;

        if Uuid::from(existing.owner_id()) != input.owner_id {
            debug!("Model belongs to a different owner, returning Forbidden");
            tx.rollback().await.map_err(UpdateModelError::from)?;
            return Err(UpdateModelError::Forbidden.into());
        }

        let updated = Model::new(
            existing.id(),
            UserId::new(input.owner_id),
            name,
            r#type,
            existing.photo_asset_id().cloned(),
        );

        debug!("Saving updated model");
        tx.save(&updated).await.map_err(UpdateModelError::from)?;

        debug!("Committing transaction");
        tx.commit().await.map_err(UpdateModelError::from)?;

        Ok(ModelDto::from(updated))
    }
}
