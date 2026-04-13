use rc_log_domain::maneuver::variation::VariationId;
use rc_log_domain::session::Session;
use rc_log_domain::session::id::SessionId;
use rc_log_domain::session::performed_variation::PerformedVariation;
use rc_log_domain::session::rating::{Comfort, Quality, Rating, Repeatability};
use rc_log_domain::session::transaction::SessionTransaction;
use rc_log_domain::shared::markdown_text::MarkdownText;
use rc_log_domain::shared::transaction::Transaction;
use rc_log_domain::shared::unit_of_work::UnitOfWork;
use tracing::{debug, instrument};
use uuid::Uuid;

use super::error::AddPerformedVariationError;
use super::model::{
    AddPerformedVariationInput, ComfortDto, QualityDto, RepeatabilityDto, SessionDto,
};
use crate::error::ApplicationError;

pub struct AddPerformedVariationUseCase<UoW> {
    uow: UoW,
}

impl<UoW> AddPerformedVariationUseCase<UoW>
where
    UoW: UnitOfWork<Session>,
    UoW::Transaction: SessionTransaction,
{
    pub fn new(uow: UoW) -> Self {
        Self { uow }
    }

    #[instrument(skip(self), fields(session_id = %input.session_id, owner_id = %input.owner_id, variation_id = %input.variation_id))]
    pub async fn execute(
        &mut self,
        input: AddPerformedVariationInput,
    ) -> Result<SessionDto, ApplicationError> {
        let quality = match input.quality {
            QualityDto::One => Quality::One,
            QualityDto::Two => Quality::Two,
            QualityDto::Three => Quality::Three,
            QualityDto::Four => Quality::Four,
            QualityDto::Five => Quality::Five,
        };

        let comfort = match input.comfort {
            ComfortDto::One => Comfort::One,
            ComfortDto::Two => Comfort::Two,
            ComfortDto::Three => Comfort::Three,
            ComfortDto::Four => Comfort::Four,
            ComfortDto::Five => Comfort::Five,
        };

        let repeatability = match input.repeatability {
            RepeatabilityDto::One => Repeatability::One,
            RepeatabilityDto::Two => Repeatability::Two,
            RepeatabilityDto::Three => Repeatability::Three,
            RepeatabilityDto::Four => Repeatability::Four,
            RepeatabilityDto::Five => Repeatability::Five,
        };

        let note = input
            .note
            .map(|n| {
                MarkdownText::new(n)
                    .map_err(|e| AddPerformedVariationError::ValidationError(e.to_string()))
            })
            .transpose()?;

        let new_performed = PerformedVariation::new(
            VariationId::new(input.variation_id),
            Rating::new(quality, comfort, repeatability),
            note,
        );

        debug!("Beginning transaction");
        let mut tx = self.uow.begin().await.map_err(AddPerformedVariationError::from)?;

        let existing = tx
            .get_by_id(SessionId::new(input.session_id))
            .await
            .map_err(AddPerformedVariationError::from)?
            .ok_or(AddPerformedVariationError::NotFound)?;

        if Uuid::from(existing.user_id()) != input.owner_id {
            tx.rollback().await.map_err(AddPerformedVariationError::from)?;
            return Err(AddPerformedVariationError::Forbidden.into());
        }

        let mut performed_variations = existing.performed_variations().to_vec();
        performed_variations.retain(|pv| Uuid::from(pv.variation_id()) != input.variation_id);
        performed_variations.push(new_performed);

        let updated = Session::new(
            existing.id(),
            existing.user_id(),
            existing.date().clone(),
            existing.model_id(),
            existing.note().cloned(),
            performed_variations,
        );

        tx.save(&updated).await.map_err(AddPerformedVariationError::from)?;
        tx.commit().await.map_err(AddPerformedVariationError::from)?;

        Ok(SessionDto::from(updated))
    }
}
