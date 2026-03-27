use rc_log_domain::user::query::UserTransaction;
use rc_log_domain::user::username::Username;
use rc_log_domain::shared::unit_of_work::UnitOfWork;
use rc_log_domain::shared::transaction::Transaction;
use rc_log_domain::user::User;
use tracing::{debug, instrument};

use crate::error::ApplicationError;
use super::error::GetUserByUsernameError;
use super::model::{GetUserByUsernameInput, UserDto};

pub struct GetUserByUsernameUseCase<UoW> {
    uow: UoW,
}

impl<UoW> GetUserByUsernameUseCase<UoW>
where
    UoW: UnitOfWork<User>,
    UoW::Transaction: UserTransaction,
{
    pub fn new(uow: UoW) -> Self {
        Self { uow }
    }

    #[instrument(skip(self), fields(username = %input.username))]
    pub async fn execute(&mut self, input: GetUserByUsernameInput) -> Result<UserDto, ApplicationError> {
        let username = Username::new(input.username)
            .map_err(|_| GetUserByUsernameError::InvalidUsername)?;

        debug!("Beginning transaction");
        let mut tx = self.uow.begin().await.map_err(GetUserByUsernameError::from)?;

        debug!("Querying user by username from repository");
        let user = tx
            .get_by_username(&username)
            .await
            .map_err(GetUserByUsernameError::from)?
            .ok_or_else(|| {
                debug!("User not found in repository");
                GetUserByUsernameError::NotFound
            })?;

        debug!("User retrieved, committing transaction");
        tx.commit().await.map_err(GetUserByUsernameError::from)?;

        Ok(UserDto::from(user))
    }
}
