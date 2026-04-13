use rc_log_domain::maneuver::maneuver_resolver::ManeuverResolver;
use rc_log_domain::maneuver::variation_resolver::VariationResolver;
use rc_log_domain::model::id::ModelId;
use rc_log_domain::model::model_resolver::ModelResolver;
use rc_log_domain::session::Session;
use rc_log_domain::session::date::Date;
use rc_log_domain::session::id::SessionId;
use rc_log_domain::session::transaction::SessionTransaction;
use rc_log_domain::shared::markdown_text::MarkdownText;
use rc_log_domain::shared::transaction::Transaction;
use rc_log_domain::shared::unit_of_work::UnitOfWork;
use tracing::{debug, instrument};
use uuid::Uuid;

use super::error::UpdateSessionError;
use super::model::{SessionDto, UpdateSessionInput};
use crate::error::ApplicationError;

pub struct UpdateSessionUseCase<UoW, MR, ManR, VarR> {
    uow: UoW,
    model_resolver: MR,
    maneuver_resolver: ManR,
    variation_resolver: VarR,
}

impl<UoW, MR, ManR, VarR> UpdateSessionUseCase<UoW, MR, ManR, VarR>
where
    UoW: UnitOfWork<Session>,
    UoW::Transaction: SessionTransaction,
    MR: ModelResolver,
    ManR: ManeuverResolver,
    VarR: VariationResolver,
{
    pub fn new(uow: UoW, model_resolver: MR, maneuver_resolver: ManR, variation_resolver: VarR) -> Self {
        Self { uow, model_resolver, maneuver_resolver, variation_resolver }
    }

    #[instrument(skip(self), fields(session_id = %input.id, owner_id = %input.owner_id, date = %input.date))]
    pub async fn execute(
        &mut self,
        input: UpdateSessionInput,
    ) -> Result<SessionDto, ApplicationError> {
        let date = Date::parse(&input.date)
            .map_err(|e| UpdateSessionError::ValidationError(e.to_string()))?;

        let note = input
            .note
            .map(|n| MarkdownText::new(n).map_err(|e| UpdateSessionError::ValidationError(e.to_string())))
            .transpose()?;

        debug!("Beginning transaction");
        let mut tx = self.uow.begin().await.map_err(UpdateSessionError::from)?;

        let existing = tx
            .get_by_id(SessionId::new(input.id))
            .await
            .map_err(UpdateSessionError::from)?
            .ok_or(UpdateSessionError::NotFound)?;

        if Uuid::from(existing.user_id()) != input.owner_id {
            tx.rollback().await.map_err(UpdateSessionError::from)?;
            return Err(UpdateSessionError::Forbidden.into());
        }

        let model_id = input.model_id.map(ModelId::new);
        if let Some(model_id) = model_id {
            let model = self
                .model_resolver
                .get_by_id(&model_id)
                .await
                .map_err(UpdateSessionError::from)?
                .ok_or(UpdateSessionError::ModelNotFound)?;

            let model_vehicle_type = model.vehicle_type();
            for performed in existing.performed_variations() {
                let variation = self
                    .variation_resolver
                    .get(performed.variation_id())
                    .await
                    .map_err(UpdateSessionError::from)?
                    .ok_or_else(|| {
                        UpdateSessionError::ValidationError(
                            "existing performed variation not found".to_string(),
                        )
                    })?;

                let maneuver = self
                    .maneuver_resolver
                    .get(variation.maneuver_id())
                    .await
                    .map_err(UpdateSessionError::from)?
                    .ok_or_else(|| {
                        UpdateSessionError::ValidationError(
                            "maneuver for existing performed variation not found".to_string(),
                        )
                    })?;

                if *maneuver.vehicle_type() != model_vehicle_type {
                    tx.rollback().await.map_err(UpdateSessionError::from)?;
                    return Err(UpdateSessionError::ValidationError(
                        "session model type must match performed variations maneuver type"
                            .to_string(),
                    )
                    .into());
                }
            }
        }

        let updated = Session::new(
            existing.id(),
            existing.user_id(),
            date,
            model_id,
            note,
            existing.performed_variations().to_vec(),
        );

        tx.save(&updated).await.map_err(UpdateSessionError::from)?;
        tx.commit().await.map_err(UpdateSessionError::from)?;

        Ok(SessionDto::from(updated))
    }
}
