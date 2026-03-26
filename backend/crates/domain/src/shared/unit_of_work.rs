use std::future::Future;

use super::transaction::{Transaction, TransactionError};

pub trait UnitOfWork<T>: Send {
    type Transaction: Transaction<T>;

    fn begin(&mut self) -> impl Future<Output = Result<Self::Transaction, TransactionError>> + Send;
}
