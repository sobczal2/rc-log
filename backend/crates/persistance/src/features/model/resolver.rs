use crate::cache::store::Cache;
use rc_log_domain::model::Model;
use rc_log_domain::model::id::ModelId;
use rc_log_domain::model::model_resolver::ModelResolver;
use rc_log_domain::model::transaction::ModelTransaction;
use rc_log_domain::shared::resolver::ResolverError;
use rc_log_domain::shared::transaction::Transaction;
use rc_log_domain::shared::transaction::TransactionError;
use rc_log_domain::shared::unit_of_work::UnitOfWork;
use sqlx::PgPool;
use uuid::Uuid;

use super::transaction::SqlxModelUnitOfWork;

fn tx_err_to_resolver(e: TransactionError) -> ResolverError {
    match e {
        TransactionError::InvalidData(msg) => ResolverError::InvalidData(msg),
        TransactionError::TransactionError(msg) => ResolverError::ResolverError(msg),
    }
}

#[derive(Clone)]
pub struct SqlxModelResolver {
    model_uow: SqlxModelUnitOfWork,
    cache: Cache<Uuid, Model>,
}

impl SqlxModelResolver {
    pub fn new(pool: PgPool, cache: Cache<Uuid, Model>) -> Self {
        Self { model_uow: SqlxModelUnitOfWork::new(pool), cache }
    }
}

impl ModelResolver for SqlxModelResolver {
    async fn get(&self, id: ModelId) -> Result<Option<Model>, ResolverError> {
        let key = id.as_uuid();

        if let Some(cached) = self.cache.get(key.clone()).await {
            return Ok(Some(cached));
        }

        let mut model_uow = self.model_uow.clone();
        let mut tx = model_uow.begin().await.map_err(tx_err_to_resolver)?;

        let model = match tx.get_by_id(id).await {
            Ok(model) => model,
            Err(err) => {
                let _ = tx.rollback().await;
                return Err(tx_err_to_resolver(err));
            }
        };

        tx.commit().await.map_err(tx_err_to_resolver)?;

        match model {
            None => Ok(None),
            Some(model) => {
                self.cache.insert(key, model.clone()).await;
                Ok(Some(model))
            }
        }
    }
}
