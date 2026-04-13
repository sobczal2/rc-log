use rc_log_domain::model::Model;
use rc_log_domain::model::id::ModelId;
use rc_log_domain::model::name::ModelName;
use rc_log_domain::model::transaction::ModelTransaction;
use rc_log_domain::shared::transaction::Transaction;
use rc_log_domain::shared::unit_of_work::UnitOfWork;
use rc_log_domain::shared::vehicle_type::VehicleType;
use rc_log_domain::user::id::UserId;
use tracing::{debug, instrument};
use uuid::Uuid;

use super::error::CreateModelError;
use super::model::{CreateModelInput, ModelDto};
use crate::error::ApplicationError;
use crate::shared::vehicle_type::VehicleTypeDto;

pub struct CreateModelUseCase<UoW> {
    uow: UoW,
}

impl<UoW> CreateModelUseCase<UoW>
where
    UoW: UnitOfWork<Model>,
    UoW::Transaction: ModelTransaction,
{
    pub fn new(uow: UoW) -> Self {
        Self { uow }
    }

    #[instrument(skip(self), fields(owner_id = %input.owner_id, name = %input.name))]
    pub async fn execute(
        &mut self,
        input: CreateModelInput,
    ) -> Result<ModelDto, ApplicationError> {
        let name = ModelName::new(input.name)
            .map_err(|e| CreateModelError::ValidationError(e.to_string()))?;

        let vehicle_type = match input.vehicle_type {
            VehicleTypeDto::Helicopter => VehicleType::Helicopter,
            VehicleTypeDto::Plane => VehicleType::Plane,
            VehicleTypeDto::Drone => VehicleType::Drone,
        };

        let model = Model::new(
            ModelId::new(Uuid::new_v4()),
            UserId::new(input.owner_id),
            name,
            vehicle_type,
            None,
        );

        debug!("Beginning transaction");
        let mut tx = self.uow.begin().await.map_err(CreateModelError::from)?;

        debug!("Saving new model");
        tx.save(&model).await.map_err(CreateModelError::from)?;

        debug!("Committing transaction");
        tx.commit().await.map_err(CreateModelError::from)?;

        Ok(ModelDto::from(model))
    }
}
