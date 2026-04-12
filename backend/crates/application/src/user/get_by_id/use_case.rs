use rc_log_domain::shared::transaction::Transaction;
use rc_log_domain::shared::unit_of_work::UnitOfWork;
use rc_log_domain::user::User;
use rc_log_domain::user::id::UserId;
use rc_log_domain::user::query::UserTransaction;
use tracing::{debug, instrument};

use super::error::GetUserByIdError;
use super::model::{GetUserByIdInput, UserDto};
use crate::error::ApplicationError;

pub struct GetUserByIdUseCase<UoW> {
    uow: UoW,
}

impl<UoW> GetUserByIdUseCase<UoW>
where
    UoW: UnitOfWork<User>,
    UoW::Transaction: UserTransaction,
{
    pub fn new(uow: UoW) -> Self {
        Self { uow }
    }

    #[instrument(skip(self), fields(user_id = %input.id))]
    pub async fn execute(&mut self, input: GetUserByIdInput) -> Result<UserDto, ApplicationError> {
        debug!("Beginning transaction");
        let mut tx = self.uow.begin().await.map_err(GetUserByIdError::from)?;

        debug!("Querying user from repository");
        let user =
            tx.get_by_id(UserId::new(input.id)).await.map_err(GetUserByIdError::from)?.ok_or_else(
                || {
                    debug!("User not found in repository");
                    GetUserByIdError::NotFound
                },
            )?;

        debug!(username = user.username().as_str(), "User retrieved, committing transaction");
        tx.commit().await.map_err(GetUserByIdError::from)?;

        Ok(UserDto::from(user))
    }
}
