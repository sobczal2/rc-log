use crate::cache::store::Cache;
use rc_log_domain::photo::resolver::PhotoResolver;
use rc_log_domain::photo::transaction::PhotoTransaction;
use rc_log_domain::photo::{Photo, PhotoId};
use rc_log_domain::shared::resolver::ResolverError;
use rc_log_domain::shared::transaction::Transaction;
use rc_log_domain::shared::transaction::TransactionError;
use rc_log_domain::shared::unit_of_work::UnitOfWork;
use sqlx::PgPool;

use super::transaction::SqlxPhotoUnitOfWork;

fn tx_err_to_resolver(e: TransactionError) -> ResolverError {
    match e {
        TransactionError::InvalidData(msg) => ResolverError::InvalidData(msg),
        TransactionError::TransactionError(msg) => ResolverError::ResolverError(msg),
    }
}

#[derive(Clone)]
pub struct SqlxPhotoResolver {
    photo_uow: SqlxPhotoUnitOfWork,
    cache: Cache<String, Photo>,
}

impl SqlxPhotoResolver {
    pub fn new(pool: PgPool, cache: Cache<String, Photo>) -> Self {
        Self { photo_uow: SqlxPhotoUnitOfWork::new(pool), cache }
    }
}

impl PhotoResolver for SqlxPhotoResolver {
    async fn get(&self, id: PhotoId) -> Result<Option<Photo>, ResolverError> {
        let key = id.as_uuid().to_string();

        if let Some(cached) = self.cache.get(key.clone()).await {
            return Ok(Some(cached));
        }

        let mut photo_uow = self.photo_uow.clone();
        let mut tx = photo_uow.begin().await.map_err(tx_err_to_resolver)?;

        let photo = match tx.get_by_id(&id).await {
            Ok(photo) => photo,
            Err(err) => {
                let _ = tx.rollback().await;
                return Err(tx_err_to_resolver(err));
            }
        };

        tx.commit().await.map_err(tx_err_to_resolver)?;

        match photo {
            None => Ok(None),
            Some(photo) => {
                self.cache.insert(key, photo.clone()).await;
                Ok(Some(photo))
            }
        }
    }
}
