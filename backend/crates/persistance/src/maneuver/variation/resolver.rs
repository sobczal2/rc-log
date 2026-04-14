use moka::future::Cache;
use rc_log_domain::maneuver::transaction::ManeuverTransaction;
use rc_log_domain::maneuver::variation::resolver::VariationResolver;
use rc_log_domain::maneuver::variation::{Variation, VariationId};
use rc_log_domain::shared::transaction::Transaction;
use rc_log_domain::shared::transaction::TransactionError;
use rc_log_domain::shared::unit_of_work::UnitOfWork;
use sqlx::PgPool;

use crate::shared::cache_settings::CacheSettings;

use super::super::transaction::SqlxManeuverUnitOfWork;

#[derive(Clone)]
pub struct SqlxVariationResolver {
    maneuver_uow: SqlxManeuverUnitOfWork,
    cache: Cache<VariationId, Variation>,
}

impl SqlxVariationResolver {
    pub fn new(pool: PgPool, settings: CacheSettings) -> Self {
        let cache =
            Cache::builder().max_capacity(settings.capacity).time_to_live(settings.ttl).build();
        Self { maneuver_uow: SqlxManeuverUnitOfWork::new(pool), cache }
    }
}

impl VariationResolver for SqlxVariationResolver {
    async fn get(&self, variation_id: VariationId) -> Result<Option<Variation>, TransactionError> {
        if let Some(cached) = self.cache.get(&variation_id).await {
            return Ok(Some(cached));
        }

        let mut maneuver_uow = self.maneuver_uow.clone();
        let mut tx = maneuver_uow.begin().await?;

        let variation = match tx.get_variation_by_id(variation_id).await {
            Ok(variation) => variation,
            Err(err) => {
                return match tx.rollback().await {
                    Ok(()) => Err(err),
                    Err(rollback_err) => Err(rollback_err),
                };
            }
        };

        tx.commit().await?;

        match variation {
            None => Ok(None),
            Some(variation) => {
                self.cache.insert(variation_id, variation.clone()).await;
                Ok(Some(variation))
            }
        }
    }
}
