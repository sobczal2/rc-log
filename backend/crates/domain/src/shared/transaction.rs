use thiserror::Error;
use std::future::Future;

#[derive(Error, Debug)]
pub enum TransactionError {
    #[error("Invalid data: {0}")]
    InvalidData(String),
    #[error("Transaction error: {0}")]
    TransactionError(String),
}

pub trait Transaction<T>: Send {
    fn save(&mut self, entity: &T) -> impl Future<Output = Result<(), TransactionError>> + Send;
    fn commit(self) -> impl Future<Output = Result<(), TransactionError>> + Send;
    fn rollback(self) -> impl Future<Output = Result<(), TransactionError>> + Send;
}
