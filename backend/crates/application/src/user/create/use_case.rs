use rc_log_domain::shared::email::Email;
use rc_log_domain::shared::password_hash::PasswordHash;
use rc_log_domain::shared::transaction::Transaction;
use rc_log_domain::shared::unit_of_work::UnitOfWork;
use rc_log_domain::user::User;
use rc_log_domain::user::username::Username;
use tracing::{debug, instrument};
use uuid::Uuid;

use super::error::CreateUserError;
use super::model::{CreateUserInput, UserDto};
use crate::error::ApplicationError;

pub struct CreateUserUseCase<UoW> {
    uow: UoW,
}

impl<UoW> CreateUserUseCase<UoW>
where
    UoW: UnitOfWork<User>,
{
    pub fn new(uow: UoW) -> Self {
        Self { uow }
    }

    #[instrument(skip(self), fields(username = %input.username))]
    pub async fn execute(&mut self, input: CreateUserInput) -> Result<UserDto, ApplicationError> {
        let username = Username::new(input.username)
            .map_err(|e| CreateUserError::ValidationError(e.to_string()))?;
        let email =
            Email::new(input.email).map_err(|e| CreateUserError::ValidationError(e.to_string()))?;
        let password_hash = PasswordHash::new(input.password_hash)
            .map_err(|e| CreateUserError::ValidationError(e.to_string()))?;

        debug!("Beginning transaction");
        let mut tx = self.uow.begin().await.map_err(CreateUserError::from)?;

        debug!("Creating new user");
        let user = User::new(Uuid::new_v4(), username, email, password_hash);

        debug!("Saving user to repository");
        tx.save(&user).await.map_err(CreateUserError::from)?;

        debug!("User saved, committing transaction");
        tx.commit().await.map_err(CreateUserError::from)?;

        Ok(UserDto::from(user))
    }
}
