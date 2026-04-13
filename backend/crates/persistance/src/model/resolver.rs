use moka::future::Cache;
use rc_log_domain::model::Model;
use rc_log_domain::model::id::ModelId;
use rc_log_domain::model::model_resolver::ModelResolver;
use rc_log_domain::model::transaction::ModelTransaction;
use rc_log_domain::shared::transaction::Transaction;
use rc_log_domain::shared::transaction::TransactionError;
use rc_log_domain::shared::unit_of_work::UnitOfWork;
use sqlx::PgPool;
use uuid::Uuid;

use crate::shared::cache_settings::CacheSettings;

use super::transaction::SqlxModelUnitOfWork;

#[derive(Clone)]
pub struct SqlxModelResolver {
    model_uow: SqlxModelUnitOfWork,
    cache: Cache<Uuid, Model>,
}

impl SqlxModelResolver {
    pub fn new(pool: PgPool, settings: CacheSettings) -> Self {
        let cache =
            Cache::builder().max_capacity(settings.capacity).time_to_live(settings.ttl).build();
        Self { model_uow: SqlxModelUnitOfWork::new(pool), cache }
    }
}

impl ModelResolver for SqlxModelResolver {
    async fn get_by_id(&self, id: &ModelId) -> Result<Option<Model>, TransactionError> {
        let key = id.as_uuid();

        if let Some(cached) = self.cache.get(&key).await {
            return Ok(Some(cached));
        }

        let mut model_uow = self.model_uow.clone();
        let mut tx = model_uow.begin().await?;

        let model = match tx.get_by_id(*id).await {
            Ok(model) => model,
            Err(err) => {
                return match tx.rollback().await {
                    Ok(()) => Err(err),
                    Err(rollback_err) => Err(rollback_err),
                };
            }
        };

        tx.commit().await?;

        match model {
            None => Ok(None),
            Some(model) => {
                self.cache.insert(key, model.clone()).await;
                Ok(Some(model))
            }
        }
    }
}
