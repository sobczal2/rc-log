use rc_log_domain::user::User;
use rc_log_domain::shared::unit_of_work::UnitOfWork;
use rc_log_domain::shared::transaction::Transaction;
use rc_log_domain::shared::password_hash::PasswordHash;
use tracing::{debug, instrument};
use uuid::Uuid;

use crate::error::ApplicationError;
use crate::shared::validator::Validate;
use super::error::CreateUserError;
use super::model::{CreateUserInput, UserDto};

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
        input.validate().map_err(|errs| {
            CreateUserError::ValidationError(
                errs.iter()
                    .map(|e| format!("{}: {}", e.field, e.message))
                    .collect::<Vec<_>>()
                    .join("; "),
            )
        })?;

        debug!("Beginning transaction");
        let mut tx = self.uow.begin().await.map_err(CreateUserError::from)?;

        debug!("Creating new user");
        let user = User::new(
            Uuid::new_v4(),
            input.username,
            input.email,
            PasswordHash::new(input.password_hash),
        );

        debug!("Saving user to repository");
        tx.save(&user).await.map_err(CreateUserError::from)?;

        debug!("User saved, committing transaction");
        tx.commit().await.map_err(CreateUserError::from)?;

        Ok(UserDto::from(user))
    }
}
