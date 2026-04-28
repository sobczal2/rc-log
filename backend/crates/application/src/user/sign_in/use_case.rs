use argon2::password_hash::PasswordHash as ArgonPasswordHash;
use argon2::{Argon2, PasswordVerifier};
use rc_log_domain::shared::transaction::Transaction;
use rc_log_domain::shared::unit_of_work::UnitOfWork;
use rc_log_domain::user::User;
use rc_log_domain::user::query::UserTransaction;
use rc_log_domain::user::username::Username;
use tracing::{debug, instrument};

use super::error::SignInError;
use super::model::{SignInInput, UserDto};

pub struct SignInUseCase<UoW> {
    uow: UoW,
}

impl<UoW> SignInUseCase<UoW>
where
    UoW: UnitOfWork<User>,
    UoW::Transaction: UserTransaction,
{
    pub fn new(uow: UoW) -> Self {
        Self { uow }
    }

    #[instrument(skip(self, input), fields(username = %input.username))]
    pub async fn execute(&mut self, input: SignInInput) -> Result<UserDto, SignInError> {
        let username =
            Username::new(input.username).map_err(|_| SignInError::InvalidCredentials)?;

        debug!("Beginning transaction");
        let mut tx = self.uow.begin().await.map_err(SignInError::from)?;

        debug!("Looking up user by username");
        let user = tx
            .get_by_username(&username)
            .await
            .map_err(SignInError::from)?
            .ok_or(SignInError::InvalidCredentials)?;

        debug!("Verifying password");
        let stored_hash = ArgonPasswordHash::new(user.password_hash().as_str())
            .map_err(|e| SignInError::InvalidData(e.to_string()))?;
        Argon2::default()
            .verify_password(input.password.as_bytes(), &stored_hash)
            .map_err(|_| SignInError::InvalidCredentials)?;

        debug!("Credentials verified, committing");
        tx.commit().await.map_err(SignInError::from)?;

        Ok(UserDto::from(user))
    }
}
