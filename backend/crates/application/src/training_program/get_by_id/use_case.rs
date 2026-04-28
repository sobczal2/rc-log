use rc_log_domain::training_program::TrainingProgram;
use rc_log_domain::training_program::id::TrainingProgramId;
use rc_log_domain::training_program::transaction::TrainingProgramTransaction;
use rc_log_domain::shared::transaction::Transaction;
use rc_log_domain::shared::unit_of_work::UnitOfWork;
use tracing::{debug, instrument};

use super::error::GetTrainingProgramByIdError;
use super::model::{GetTrainingProgramByIdInput, TrainingProgramDto};

pub struct GetTrainingProgramByIdUseCase<UoW> {
    uow: UoW,
}

impl<UoW> GetTrainingProgramByIdUseCase<UoW>
where
    UoW: UnitOfWork<TrainingProgram>,
    UoW::Transaction: TrainingProgramTransaction,
{
    pub fn new(uow: UoW) -> Self {
        Self { uow }
    }

    #[instrument(skip(self), fields(training_program_id = %input.id))]
    pub async fn execute(
        &mut self,
        input: GetTrainingProgramByIdInput,
    ) -> Result<TrainingProgramDto, GetTrainingProgramByIdError> {
        debug!("Beginning transaction");
        let mut tx = self.uow.begin().await.map_err(GetTrainingProgramByIdError::from)?;

        debug!("Querying training program from repository");
        let training_program = tx
            .get_by_id(TrainingProgramId::new(input.id))
            .await
            .map_err(GetTrainingProgramByIdError::from)?
            .ok_or_else(|| {
                debug!("Training program not found");
                GetTrainingProgramByIdError::NotFound
            })?;

        debug!("Training program retrieved, committing transaction");
        tx.commit().await.map_err(GetTrainingProgramByIdError::from)?;

        Ok(TrainingProgramDto::from(training_program))
    }
}
