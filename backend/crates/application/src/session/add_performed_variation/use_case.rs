use rc_log_domain::maneuver::resolver::ManeuverResolver;
use rc_log_domain::maneuver::variation::VariationId;
use rc_log_domain::maneuver::variation::resolver::VariationResolver;
use rc_log_domain::model::Type;
use rc_log_domain::model::model_resolver::ModelResolver;
use rc_log_domain::session::Session;
use rc_log_domain::session::id::SessionId;
use rc_log_domain::session::performed_variation::PerformedVariation;
use rc_log_domain::session::performed_variation::id::PerformedVariationId;
use rc_log_domain::session::transaction::SessionTransaction;
use rc_log_domain::shared::markdown_text::MarkdownText;
use rc_log_domain::shared::transaction::Transaction;
use rc_log_domain::shared::unit_of_work::UnitOfWork;
use tracing::{debug, instrument};
use uuid::Uuid;

use super::error::AddPerformedVariationError;
use super::model::{AddPerformedVariationInput, PerformedVariationDto};
use crate::error::ApplicationError;
use crate::session::shared::rating::rating_from_dto;

pub struct AddPerformedVariationUseCase<UoW, MR, ManR, VarR> {
    uow: UoW,
    model_resolver: MR,
    maneuver_resolver: ManR,
    variation_resolver: VarR,
}

impl<UoW, MR, ManR, VarR> AddPerformedVariationUseCase<UoW, MR, ManR, VarR>
where
    UoW: UnitOfWork<Session>,
    UoW::Transaction: SessionTransaction,
    MR: ModelResolver,
    ManR: ManeuverResolver,
    VarR: VariationResolver,
{
    pub fn new(
        uow: UoW,
        model_resolver: MR,
        maneuver_resolver: ManR,
        variation_resolver: VarR,
    ) -> Self {
        Self { uow, model_resolver, maneuver_resolver, variation_resolver }
    }

    #[instrument(skip(self), fields(session_id = %input.session_id, owner_id = %input.owner_id, variation_id = %input.variation_id))]
    pub async fn execute(
        &mut self,
        input: AddPerformedVariationInput,
    ) -> Result<PerformedVariationDto, ApplicationError> {
        let performed_variation_id = Uuid::new_v4();

        let quality = rating_from_dto(input.quality);
        let comfort = rating_from_dto(input.comfort);
        let repeatability = rating_from_dto(input.repeatability);

        let note = input
            .note
            .map(|n| {
                MarkdownText::new(n)
                    .map_err(|e| AddPerformedVariationError::ValidationError(e.to_string()))
            })
            .transpose()?;

        let new_performed = PerformedVariation::new(
            PerformedVariationId::new(performed_variation_id),
            VariationId::new(input.variation_id),
            quality,
            comfort,
            repeatability,
            note,
        );
        let created_dto = PerformedVariationDto::from(new_performed.clone());

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

        let variation = self
            .variation_resolver
            .get(VariationId::new(input.variation_id))
            .await
            .map_err(AddPerformedVariationError::from)?
            .ok_or_else(|| {
                AddPerformedVariationError::ValidationError("variation not found".to_string())
            })?;

        let variation_maneuver = self
            .maneuver_resolver
            .get(variation.maneuver_id())
            .await
            .map_err(AddPerformedVariationError::from)?
            .ok_or_else(|| {
                AddPerformedVariationError::ValidationError(
                    "maneuver for variation not found".to_string(),
                )
            })?;
        let added_model_type = *variation_maneuver.model_type();

        if let Some(model_id) = existing.model_id() {
            let model = self
                .model_resolver
                .get_by_id(&model_id)
                .await
                .map_err(AddPerformedVariationError::from)?
                .ok_or_else(|| {
                    AddPerformedVariationError::ValidationError(
                        "session model not found".to_string(),
                    )
                })?;

            if model.r#type() != added_model_type {
                tx.rollback().await.map_err(AddPerformedVariationError::from)?;
                return Err(AddPerformedVariationError::ValidationError(
                    "variation maneuver type must match session model type".to_string(),
                )
                .into());
            }
        } else if let Some(first_existing) = existing.performed_variations().first() {
            let first_variation = self
                .variation_resolver
                .get(first_existing.variation_id())
                .await
                .map_err(AddPerformedVariationError::from)?
                .ok_or_else(|| {
                    AddPerformedVariationError::ValidationError(
                        "existing performed variation not found".to_string(),
                    )
                })?;

            let first_maneuver = self
                .maneuver_resolver
                .get(first_variation.maneuver_id())
                .await
                .map_err(AddPerformedVariationError::from)?
                .ok_or_else(|| {
                    AddPerformedVariationError::ValidationError(
                        "maneuver for existing performed variation not found".to_string(),
                    )
                })?;

            let first_model_type: Type = *first_maneuver.model_type();

            if first_model_type != added_model_type {
                tx.rollback().await.map_err(AddPerformedVariationError::from)?;
                return Err(AddPerformedVariationError::ValidationError(
                    "variation maneuver type must match existing performed variations type"
                        .to_string(),
                )
                .into());
            }
        }

        let mut performed_variations = existing.performed_variations().to_vec();
        performed_variations.push(new_performed);
        performed_variations.sort_by_key(|pv| pv.id().as_uuid());

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

        Ok(created_dto)
    }
}
