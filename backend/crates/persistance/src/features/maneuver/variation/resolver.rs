use crate::cache::store::Cache;
use rc_log_domain::maneuver::transaction::ManeuverTransaction;
use rc_log_domain::maneuver::variation::resolver::VariationResolver;
use rc_log_domain::maneuver::variation::{Variation, VariationId};
use rc_log_domain::shared::resolver::ResolverError;
use rc_log_domain::shared::transaction::Transaction;
use rc_log_domain::shared::transaction::TransactionError;
use rc_log_domain::shared::unit_of_work::UnitOfWork;
use sqlx::PgPool;

use super::super::transaction::SqlxManeuverUnitOfWork;

fn tx_err_to_resolver(e: TransactionError) -> ResolverError {
    match e {
        TransactionError::InvalidData(msg) => ResolverError::InvalidData(msg),
        TransactionError::TransactionError(msg) => ResolverError::ResolverError(msg),
    }
}

#[derive(Clone)]
pub struct SqlxVariationResolver {
    maneuver_uow: SqlxManeuverUnitOfWork,
    cache: Cache<VariationId, Variation>,
}

impl SqlxVariationResolver {
    pub fn new(pool: PgPool, cache: Cache<VariationId, Variation>) -> Self {
        Self { maneuver_uow: SqlxManeuverUnitOfWork::new(pool), cache }
    }
}

impl VariationResolver for SqlxVariationResolver {
    async fn get(&self, variation_id: VariationId) -> Result<Option<Variation>, ResolverError> {
        if let Some(cached) = self.cache.get(variation_id.clone()).await {
            return Ok(Some(cached));
        }

        let mut maneuver_uow = self.maneuver_uow.clone();
        let mut tx = maneuver_uow.begin().await.map_err(tx_err_to_resolver)?;

        let variation = match tx.get_variation_by_id(variation_id).await {
            Ok(variation) => variation,
            Err(err) => {
                let _ = tx.rollback().await;
                return Err(tx_err_to_resolver(err));
            }
        };

        tx.commit().await.map_err(tx_err_to_resolver)?;

        match variation {
            None => Ok(None),
            Some(variation) => {
                self.cache.insert(variation_id, variation.clone()).await;
                Ok(Some(variation))
            }
        }
    }
}
