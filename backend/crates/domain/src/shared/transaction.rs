use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::future::Future;

#[derive(Debug)]
pub enum TransactionError {
    InvalidData(String),
    TransactionError(String),
}

impl Display for TransactionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            TransactionError::InvalidData(msg) => write!(f, "Invalid data: {}", msg),
            TransactionError::TransactionError(msg) => write!(f, "Transaction error: {}", msg),
        }
    }
}

impl Error for TransactionError {}

pub trait Transaction<T>: Send {
    fn save(&mut self, entity: &T) -> impl Future<Output = Result<(), TransactionError>> + Send;
    fn commit(self) -> impl Future<Output = Result<(), TransactionError>> + Send;
    fn rollback(self) -> impl Future<Output = Result<(), TransactionError>> + Send;
}
