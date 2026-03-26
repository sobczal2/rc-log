use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::future::Future;

use uuid::Uuid;

use crate::shared::pagination::Pagination;

#[derive(Debug)]
pub enum RepositoryError {
    InvalidData(String),
    TransactionError(String),
}

impl Display for RepositoryError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            RepositoryError::InvalidData(msg) => write!(f, "Invalid data: {}", msg),
            RepositoryError::TransactionError(msg) => write!(f, "Transaction error: {}", msg),
        }
    }
}

impl Error for RepositoryError {}

pub trait Transaction<T>: Send {
    type Filter: Send + Default;
    type Sort: Send + Default;

    fn get_by_id(
        &mut self,
        id: Uuid,
    ) -> impl Future<Output = Result<Option<T>, RepositoryError>> + Send;
    
    fn list(
        &mut self,
        pagination: Pagination,
        filter: Self::Filter,
        sort: Self::Sort,
    ) -> impl Future<Output = Result<(Vec<T>, u64), RepositoryError>> + Send;
    
    fn save(&mut self, entity: &T) -> impl Future<Output = Result<(), RepositoryError>> + Send;
    fn commit(self) -> impl Future<Output = Result<(), RepositoryError>> + Send;
    fn rollback(self) -> impl Future<Output = Result<(), RepositoryError>> + Send;
}

pub trait UnitOfWork<T>: Send {
    type Transaction: Transaction<T>;

    fn begin(&mut self) -> impl Future<Output = Result<Self::Transaction, RepositoryError>> + Send;
}
