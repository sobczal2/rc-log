use rc_log_domain::model::id::ModelId;
use rc_log_domain::model::model_resolver::ModelResolver;
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

pub struct CreateSessionUseCase<SessionUoW, MR> {
    session_uow: SessionUoW,
    model_resolver: MR,
}

impl<SessionUoW, MR> CreateSessionUseCase<SessionUoW, MR>
where
    SessionUoW: UnitOfWork<Session>,
    SessionUoW::Transaction: SessionTransaction,
    MR: ModelResolver,
{
    pub fn new(session_uow: SessionUoW, model_resolver: MR) -> Self {
        Self { session_uow, model_resolver }
    }

    #[instrument(skip(self), fields(user_id = %input.user_id, date = %input.date))]
    pub async fn execute(
        &mut self,
        input: CreateSessionInput,
    ) -> Result<SessionDto, CreateSessionError> {
        let model_id = input.model_id.map(ModelId::new);

        if let Some(model_id) = model_id {
            debug!(model_id = %model_id.as_uuid(), "Checking if referenced model exists");
            let model = self
                .model_resolver
                .get(model_id)
                .await
                .map_err(CreateSessionError::from)?;

            if model.is_none() {
                debug!(model_id = %model_id.as_uuid(), "Referenced model not found");
                return Err(CreateSessionError::ModelNotFound);
            }
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
