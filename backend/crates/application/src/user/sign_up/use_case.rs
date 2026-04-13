use argon2::password_hash::{SaltString, rand_core::OsRng};
use argon2::{Argon2, PasswordHasher};
use rc_log_domain::shared::email::Email;
use rc_log_domain::shared::password_hash::PasswordHash;
use rc_log_domain::shared::transaction::Transaction;
use rc_log_domain::shared::unit_of_work::UnitOfWork;
use rc_log_domain::user::User;
use rc_log_domain::user::id::UserId;
use rc_log_domain::user::query::UserTransaction;
use rc_log_domain::user::username::Username;
use tracing::{debug, instrument};
use uuid::Uuid;

use super::error::SignUpError;
use super::model::{SignUpInput, UserDto};
use crate::error::ApplicationError;

pub struct SignUpUseCase<UoW> {
    uow: UoW,
}

impl<UoW> SignUpUseCase<UoW>
where
    UoW: UnitOfWork<User>,
    UoW::Transaction: UserTransaction,
{
    pub fn new(uow: UoW) -> Self {
        Self { uow }
    }

    #[instrument(skip(self, input), fields(username = %input.username))]
    pub async fn execute(&mut self, input: SignUpInput) -> Result<UserDto, ApplicationError> {
        let username = Username::new(input.username)
            .map_err(|e| SignUpError::ValidationError(e.to_string()))?;
        let email =
            Email::new(input.email).map_err(|e| SignUpError::ValidationError(e.to_string()))?;

        debug!("Hashing password");
        let salt = SaltString::generate(&mut OsRng);
        let hash_string = Argon2::default()
            .hash_password(input.password.as_bytes(), &salt)
            .map_err(|e| SignUpError::HashingError(e.to_string()))?
            .to_string();
        let password_hash = PasswordHash::new(hash_string)
            .map_err(|e| SignUpError::ValidationError(e.to_string()))?;

        debug!("Beginning transaction");
        let mut tx = self.uow.begin().await.map_err(SignUpError::from)?;

        debug!("Checking username availability");
        if tx.get_by_username(&username).await.map_err(SignUpError::from)?.is_some() {
            tx.rollback().await.map_err(SignUpError::from)?;
            return Err(SignUpError::UsernameTaken.into());
        }

        debug!("Checking email availability");
        if tx.get_by_email(&email).await.map_err(SignUpError::from)?.is_some() {
            tx.rollback().await.map_err(SignUpError::from)?;
            return Err(SignUpError::EmailTaken.into());
        }

        debug!("Saving new user");
        let user = User::new(UserId::new(Uuid::new_v4()), username, email, password_hash);
        tx.save(&user).await.map_err(SignUpError::from)?;

        debug!("Committing transaction");
        tx.commit().await.map_err(SignUpError::from)?;

        Ok(UserDto::from(user))
    }
}
