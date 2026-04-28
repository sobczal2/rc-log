use rc_log_domain::session::Session;
use rc_log_domain::session::id::SessionId;
use rc_log_domain::session::performed_variation::PerformedVariation;
use rc_log_domain::session::transaction::SessionTransaction;
use rc_log_domain::shared::markdown_text::MarkdownText;
use rc_log_domain::shared::transaction::Transaction;
use rc_log_domain::shared::unit_of_work::UnitOfWork;
use tracing::{debug, instrument};
use uuid::Uuid;

use super::error::UpdatePerformedVariationError;
use super::model::UpdatePerformedVariationInput;
use crate::session::shared::rating::rating_from_dto;

pub struct UpdatePerformedVariationUseCase<UoW> {
    uow: UoW,
}

impl<UoW> UpdatePerformedVariationUseCase<UoW>
where
    UoW: UnitOfWork<Session>,
    UoW::Transaction: SessionTransaction,
{
    pub fn new(uow: UoW) -> Self {
        Self { uow }
    }

    #[instrument(skip(self), fields(session_id = %input.session_id, owner_id = %input.owner_id, performed_variation_id = %input.performed_variation_id))]
    pub async fn execute(
        &mut self,
        input: UpdatePerformedVariationInput,
    ) -> Result<(), UpdatePerformedVariationError> {
        let quality = rating_from_dto(input.quality);
        let comfort = rating_from_dto(input.comfort);
        let repeatability = rating_from_dto(input.repeatability);

        let note = input
            .note
            .map(|n| {
                MarkdownText::new(n)
                    .map_err(|e| UpdatePerformedVariationError::ValidationError(e.to_string()))
            })
            .transpose()?;

        debug!("Beginning transaction");
        let mut tx = self.uow.begin().await.map_err(UpdatePerformedVariationError::from)?;

        let existing = tx
            .get_by_id(SessionId::new(input.session_id))
            .await
            .map_err(UpdatePerformedVariationError::from)?
            .ok_or(UpdatePerformedVariationError::NotFound)?;

        if Uuid::from(existing.user_id()) != input.owner_id {
            tx.rollback().await.map_err(UpdatePerformedVariationError::from)?;
            return Err(UpdatePerformedVariationError::Forbidden);
        }

        let mut found = false;
        let mut performed_variations = existing
            .performed_variations()
            .iter()
            .map(|pv| {
                if Uuid::from(pv.id()) == input.performed_variation_id {
                    found = true;
                    PerformedVariation::new(
                        pv.id(),
                        pv.variation_id(),
                        quality,
                        comfort,
                        repeatability,
                        note.clone(),
                    )
                } else {
                    pv.clone()
                }
            })
            .collect::<Vec<_>>();

        if !found {
            tx.rollback().await.map_err(UpdatePerformedVariationError::from)?;
            return Err(UpdatePerformedVariationError::PerformedVariationNotFound);
        }

        performed_variations.sort_by_key(|pv| pv.id().as_uuid());

        let updated = Session::new(
            existing.id(),
            existing.user_id(),
            existing.date().clone(),
            existing.model_id(),
            existing.note().cloned(),
            performed_variations,
        );

        tx.save(&updated).await.map_err(UpdatePerformedVariationError::from)?;
        tx.commit().await.map_err(UpdatePerformedVariationError::from)?;

        Ok(())
    }
}
