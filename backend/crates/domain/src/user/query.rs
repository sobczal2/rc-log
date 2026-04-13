use std::future::Future;

use crate::shared::email::Email;
use crate::shared::transaction::{Transaction, TransactionError};
use crate::user::User;
use crate::user::id::UserId;
use crate::user::username::Username;

/// Transaction trait extended with User-specific operations
pub trait UserTransaction: Transaction<User> {
    fn get_by_id(
        &mut self,
        id: UserId,
    ) -> impl Future<Output = Result<Option<User>, TransactionError>> + Send;

    fn get_by_username(
        &mut self,
        username: &Username,
    ) -> impl Future<Output = Result<Option<User>, TransactionError>> + Send;

    fn get_by_email(
        &mut self,
        email: &Email,
    ) -> impl Future<Output = Result<Option<User>, TransactionError>> + Send;
}
