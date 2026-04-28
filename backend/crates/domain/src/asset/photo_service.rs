use thiserror::Error;
use std::future::Future;

use super::photo::{Photo, PhotoId};

#[derive(Error, Debug)]
pub enum PhotoServiceError {
    #[error("I/O error: {0}")]
    IoError(String),
    #[error("Database error: {0}")]
    DatabaseError(String),
    #[error("Invalid data: {0}")]
    InvalidData(String),
}

pub trait PhotoService: Send + Sync + Clone {
    fn save(
        &self,
        id: &PhotoId,
        data: &[u8],
    ) -> impl Future<Output = Result<Photo, PhotoServiceError>> + Send;

    fn delete(&self, id: &PhotoId) -> impl Future<Output = Result<(), PhotoServiceError>> + Send;
}
