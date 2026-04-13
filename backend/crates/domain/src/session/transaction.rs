use std::future::Future;

use crate::session::Session;
use crate::session::id::SessionId;
use crate::shared::transaction::{Transaction, TransactionError};

pub trait SessionTransaction: Transaction<Session> {
    fn get_by_id(
        &mut self,
        id: SessionId,
    ) -> impl Future<Output = Result<Option<Session>, TransactionError>> + Send;
}