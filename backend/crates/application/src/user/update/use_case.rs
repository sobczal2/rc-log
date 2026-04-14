use rc_log_domain::shared::transaction::Transaction;
use rc_log_domain::shared::unit_of_work::UnitOfWork;
use rc_log_domain::user::User;
use rc_log_domain::user::id::UserId;
use rc_log_domain::user::query::UserTransaction;
use rc_log_domain::user::username::Username;
use tracing::{debug, instrument};

use super::error::UpdateUserError;
use super::model::{UpdateUserInput, UserDto};
use crate::error::ApplicationError;

pub struct UpdateUserUseCase<UoW> {
    uow: UoW,
}

impl<UoW> UpdateUserUseCase<UoW>
where
    UoW: UnitOfWork<User>,
    UoW::Transaction: UserTransaction,
{
    pub fn new(uow: UoW) -> Self {
        Self { uow }
    }

    #[instrument(skip(self), fields(user_id = %input.user_id, new_username = %input.new_username))]
    pub async fn execute(&mut self, input: UpdateUserInput) -> Result<UserDto, ApplicationError> {
        let new_username = Username::new(input.new_username)
            .map_err(|e| UpdateUserError::ValidationError(e.to_string()))?;

        debug!("Beginning transaction");
        let mut tx = self.uow.begin().await.map_err(UpdateUserError::from)?;

        debug!("Checking username availability");
        if let Some(existing_by_name) =
            tx.get_by_username(&new_username).await.map_err(UpdateUserError::from)?
        {
            if existing_by_name.id() != UserId::new(input.user_id) {
                tx.rollback().await.map_err(UpdateUserError::from)?;
                return Err(UpdateUserError::UsernameTaken.into());
            }
        }

        debug!("Fetching user from repository");
        let user = tx
            .get_by_id(UserId::new(input.user_id))
            .await
            .map_err(UpdateUserError::from)?
            .ok_or_else(|| {
            debug!("User not found");
            UpdateUserError::NotFound
        })?;

        let updated = User::new(
            user.id(),
            new_username,
            user.email().clone(),
            user.password_hash().clone(),
            user.photo_asset_name().cloned(),
        );

        debug!("Saving updated user");
        tx.save(&updated).await.map_err(UpdateUserError::from)?;

        debug!("Committing transaction");
        tx.commit().await.map_err(UpdateUserError::from)?;

        Ok(UserDto::from(updated))
    }
}
