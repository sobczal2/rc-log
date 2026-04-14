use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::future::Future;

use super::name::Name;
use super::photo::Photo;

#[derive(Debug)]
pub enum PhotoServiceError {
    IoError(String),
    DatabaseError(String),
    InvalidData(String),
}

impl Display for PhotoServiceError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            PhotoServiceError::IoError(msg) => write!(f, "I/O error: {}", msg),
            PhotoServiceError::DatabaseError(msg) => write!(f, "Database error: {}", msg),
            PhotoServiceError::InvalidData(msg) => write!(f, "Invalid data: {}", msg),
        }
    }
}

impl Error for PhotoServiceError {}

pub trait PhotoService: Send + Sync + Clone {
    fn save(
        &self,
        name: &Name,
        data: &[u8],
    ) -> impl Future<Output = Result<Photo, PhotoServiceError>> + Send;

    fn delete(
        &self,
        name: &Name,
    ) -> impl Future<Output = Result<(), PhotoServiceError>> + Send;
}
