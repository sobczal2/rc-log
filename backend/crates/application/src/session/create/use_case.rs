use rc_log_domain::model::id::ModelId;
use rc_log_domain::model::transaction::ModelTransaction;
use rc_log_domain::session::Session;
use rc_log_domain::session::date::Date;
use rc_log_domain::session::id::SessionId;
use rc_log_domain::session::transaction::SessionTransaction;
use rc_log_domain::shared::markdown_text::MarkdownText;
use rc_log_domain::shared::transaction::Transaction;
use rc_log_domain::shared::unit_of_work::UnitOfWork;
use rc_log_domain::user::id::UserId;
use tracing::{debug, instrument};
use uuid::Uuid;

use super::error::CreateSessionError;
use super::model::{CreateSessionInput, SessionDto};
use crate::error::ApplicationError;

pub struct CreateSessionUseCase<SessionUoW, ModelUoW> {
    session_uow: SessionUoW,
    model_uow: ModelUoW,
}

impl<SessionUoW, ModelUoW> CreateSessionUseCase<SessionUoW, ModelUoW>
where
    SessionUoW: UnitOfWork<Session>,
    SessionUoW::Transaction: SessionTransaction,
    ModelUoW: UnitOfWork<rc_log_domain::model::Model>,
    ModelUoW::Transaction: ModelTransaction,
{
    pub fn new(session_uow: SessionUoW, model_uow: ModelUoW) -> Self {
        Self { session_uow, model_uow }
    }

    #[instrument(skip(self), fields(user_id = %input.user_id, date = %input.date))]
    pub async fn execute(
        &mut self,
        input: CreateSessionInput,
    ) -> Result<SessionDto, ApplicationError> {
        let model_id = input.model_id.map(ModelId::new);

        if let Some(model_id) = model_id {
            debug!(model_id = %model_id.as_uuid(), "Checking if referenced model exists");
            let mut model_tx = self.model_uow.begin().await.map_err(CreateSessionError::from)?;
            let model = model_tx.get_by_id(model_id).await.map_err(CreateSessionError::from)?;

            if model.is_none() {
                debug!(model_id = %model_id.as_uuid(), "Referenced model not found");
                model_tx.rollback().await.map_err(CreateSessionError::from)?;
                return Err(CreateSessionError::ModelNotFound.into());
            }

            model_tx.commit().await.map_err(CreateSessionError::from)?;
        }

        let date = Date::parse(&input.date)
            .map_err(|e| CreateSessionError::ValidationError(e.to_string()))?;

        let note = input
            .note
            .map(|n| {
                MarkdownText::new(n).map_err(|e| CreateSessionError::ValidationError(e.to_string()))
            })
            .transpose()?;

        let session = Session::new(
            SessionId::new(Uuid::new_v4()),
            UserId::new(input.user_id),
            date,
            model_id,
            note,
            Vec::new(),
        );

        debug!("Beginning transaction");
        let mut tx = self.session_uow.begin().await.map_err(CreateSessionError::from)?;

        debug!("Saving new session");
        tx.save(&session).await.map_err(CreateSessionError::from)?;

        debug!("Committing transaction");
        tx.commit().await.map_err(CreateSessionError::from)?;

        Ok(SessionDto::from(session))
    }
}
