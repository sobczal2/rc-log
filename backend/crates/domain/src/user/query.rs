use std::future::Future;

use crate::shared::transaction::{Transaction, TransactionError};
use crate::user::User;

/// Transaction trait extended with User-specific operations
pub trait UserTransaction: Transaction<User> {
    fn get_by_username(
        &mut self,
        username: &str,
    ) -> impl Future<Output = Result<Option<User>, TransactionError>> + Send;
}
