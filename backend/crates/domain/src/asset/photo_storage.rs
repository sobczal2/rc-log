use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::future::Future;

use super::name::AssetName;
use super::photo::Photo;

#[derive(Debug)]
pub enum PhotoStorageError {
    IoError(String),
    DatabaseError(String),
    InvalidData(String),
}

impl Display for PhotoStorageError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            PhotoStorageError::IoError(msg) => write!(f, "I/O error: {}", msg),
            PhotoStorageError::DatabaseError(msg) => write!(f, "Database error: {}", msg),
            PhotoStorageError::InvalidData(msg) => write!(f, "Invalid data: {}", msg),
        }
    }
}

impl Error for PhotoStorageError {}

pub trait PhotoStorage: Send + Sync + Clone {
    fn store(
        &self,
        name: &AssetName,
        data: &[u8],
    ) -> impl Future<Output = Result<Photo, PhotoStorageError>> + Send;

    fn delete(
        &self,
        name: &AssetName,
    ) -> impl Future<Output = Result<(), PhotoStorageError>> + Send;
}
