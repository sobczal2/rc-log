use moka::future::Cache;
use rc_log_domain::asset::name::AssetName;
use rc_log_domain::asset::photo::Photo;
use rc_log_domain::asset::photo_resolver::PhotoResolver;
use rc_log_domain::asset::photo_transaction::PhotoTransaction;
use rc_log_domain::shared::transaction::Transaction;
use rc_log_domain::shared::transaction::TransactionError;
use rc_log_domain::shared::unit_of_work::UnitOfWork;
use crate::shared::cache_settings::CacheSettings;
use sqlx::PgPool;

use super::transaction::SqlxPhotoUnitOfWork;

#[derive(Clone)]
pub struct SqlxPhotoResolver {
    photo_uow: SqlxPhotoUnitOfWork,
    cache: Cache<String, Photo>,
}

impl SqlxPhotoResolver {
    pub fn new(pool: PgPool, settings: CacheSettings) -> Self {
        let cache = Cache::builder().max_capacity(settings.capacity).time_to_live(settings.ttl).build();
        Self { photo_uow: SqlxPhotoUnitOfWork::new(pool), cache }
    }
}

impl PhotoResolver for SqlxPhotoResolver {
    async fn get(&self, name: &AssetName) -> Result<Option<Photo>, TransactionError> {
        let key = name.as_str().to_string();

        if let Some(cached) = self.cache.get(&key).await {
            return Ok(Some(cached));
        }

        let mut photo_uow = self.photo_uow.clone();
        let mut tx = photo_uow.begin().await?;

        let photo = match tx.get_by_name(name).await {
            Ok(photo) => photo,
            Err(err) => {
                return match tx.rollback().await {
                    Ok(()) => Err(err),
                    Err(rollback_err) => Err(rollback_err),
                };
            }
        };

        tx.commit().await?;

        match photo {
            None => Ok(None),
            Some(photo) => {
                self.cache.insert(key, photo.clone()).await;
                Ok(Some(photo))
            }
        }
    }
}
