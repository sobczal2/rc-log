use rc_log_domain::maneuver::resolver::ManeuverResolver;
use rc_log_domain::maneuver::variation::resolver::VariationResolver;
use rc_log_domain::model::Type;
use rc_log_domain::model::model_resolver::ModelResolver;
use rc_log_domain::session::Session;
use rc_log_domain::session::transaction::SessionTransaction;
use rc_log_domain::shared::pagination::Pagination;
use rc_log_domain::shared::transaction::Transaction;
use rc_log_domain::shared::unit_of_work::UnitOfWork;
use rc_log_domain::user::id::UserId;
use tracing::{debug, instrument};

use super::error::ListSessionsError;
use super::model::{
    ListSessionsInput, PerformedVariationDto, SessionDto, comfort_to_dto, quality_to_dto,
    repeatability_to_dto,
};
use crate::error::ApplicationError;
use crate::shared::TypeDto;
use crate::shared::pagination::PaginatedResult;

pub struct ListSessionsUseCase<UoW, MR, ManR, VarR> {
    uow: UoW,
    model_resolver: MR,
    maneuver_resolver: ManR,
    variation_resolver: VarR,
}

impl<UoW, MR, ManR, VarR> ListSessionsUseCase<UoW, MR, ManR, VarR>
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

    #[instrument(skip(self), fields(owner_id = %input.owner_id, page = input.pagination.page, page_size = input.pagination.page_size))]
    pub async fn execute(
        &mut self,
        input: ListSessionsInput,
    ) -> Result<PaginatedResult<SessionDto>, ApplicationError> {
        debug!("Beginning transaction");
        let mut tx = self.uow.begin().await.map_err(ListSessionsError::from)?;

        let page = input.pagination.page;
        let page_size = input.pagination.page_size;
        let owner_id = UserId::new(input.owner_id);
        let pagination = Pagination::from(input.pagination);
        let filter = input.filter.into();
        let sort = input.sort.into();

        debug!("Querying sessions from repository");
        let (sessions, total) = tx
            .list_by_owner(owner_id, pagination, filter, sort)
            .await
            .map_err(ListSessionsError::from)?;

        debug!(count = sessions.len(), total, "Sessions retrieved, committing transaction");
        tx.commit().await.map_err(ListSessionsError::from)?;

        let mut items = Vec::with_capacity(sessions.len());
        for session in sessions {
            let (model_name, mut model_type, model_photo_asset_id) = match session.model_id() {
                None => (None, None, None),
                Some(model_id) => {
                    let model = self
                        .model_resolver
                        .get_by_id(&model_id)
                        .await
                        .map_err(ListSessionsError::from)?;

                    match model {
                        None => (None, None, None),
                        Some(model) => {
                            let model_type = match model.r#type() {
                                Type::Helicopter => TypeDto::Helicopter,
                                Type::Plane => TypeDto::Plane,
                                Type::Drone => TypeDto::Drone,
                            };
                            (
                                Some(model.name().as_str().to_string()),
                                Some(model_type),
                                model.photo_asset_id().map(|id| id.as_uuid().to_string()),
                            )
                        }
                    }
                }
            };

            let mut inferred_model_type_from_variations = None;
            let mut performed_variations = Vec::with_capacity(session.performed_variations().len());
            for performed in session.performed_variations() {
                let variation_id = performed.variation_id();
                let variation = self
                    .variation_resolver
                    .get(variation_id)
                    .await
                    .map_err(ListSessionsError::from)?;

                let (variation_name, maneuver_name) = match variation {
                    None => (None, None),
                    Some(variation) => {
                        let variation_name = Some(variation.name().to_string());
                        let maneuver = self
                            .maneuver_resolver
                            .get(variation.maneuver_id())
                            .await
                            .map_err(ListSessionsError::from)?;

                        if inferred_model_type_from_variations.is_none() {
                            if let Some(ref m) = maneuver {
                                inferred_model_type_from_variations = Some(match *m.model_type() {
                                    Type::Helicopter => TypeDto::Helicopter,
                                    Type::Plane => TypeDto::Plane,
                                    Type::Drone => TypeDto::Drone,
                                });
                            }
                        }

                        let maneuver_name = maneuver.map(|m| m.name().to_string());
                        (variation_name, maneuver_name)
                    }
                };

                let rating = performed.rating();
                performed_variations.push(PerformedVariationDto {
                    performed_variation_id: performed.id().as_uuid(),
                    variation_id: variation_id.as_uuid(),
                    maneuver_name,
                    variation_name,
                    quality: quality_to_dto(rating.quality()),
                    comfort: comfort_to_dto(rating.comfort()),
                    repeatability: repeatability_to_dto(rating.repeatability()),
                });
            }

            performed_variations.sort_by_key(|pv| pv.performed_variation_id);

            if model_type.is_none() {
                model_type = inferred_model_type_from_variations;
            }

            items.push(SessionDto {
                id: session.id().as_uuid(),
                user_id: session.user_id().as_uuid(),
                date: session.date().as_naive_date().format("%Y-%m-%d").to_string(),
                model_id: session.model_id().map(|id| id.as_uuid()),
                model_name,
                model_type,
                model_photo_asset_id,
                performed_variations,
            });
        }

        Ok(PaginatedResult::new(items, total, page, page_size))
    }
}
