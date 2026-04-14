use moka::future::Cache;
use rc_log_domain::maneuver::Maneuver;
use rc_log_domain::maneuver::id::ManeuverId;
use rc_log_domain::maneuver::resolver::ManeuverResolver;
use rc_log_domain::maneuver::transaction::ManeuverTransaction;
use rc_log_domain::shared::transaction::Transaction;
use rc_log_domain::shared::transaction::TransactionError;
use rc_log_domain::shared::unit_of_work::UnitOfWork;
use sqlx::PgPool;

use crate::shared::cache_settings::CacheSettings;

use super::transaction::SqlxManeuverUnitOfWork;

#[derive(Clone)]
pub struct SqlxManeuverResolver {
    maneuver_uow: SqlxManeuverUnitOfWork,
    cache: Cache<ManeuverId, Maneuver>,
}

impl SqlxManeuverResolver {
    pub fn new(pool: PgPool, settings: CacheSettings) -> Self {
        let cache =
            Cache::builder().max_capacity(settings.capacity).time_to_live(settings.ttl).build();
        Self { maneuver_uow: SqlxManeuverUnitOfWork::new(pool), cache }
    }
}

impl ManeuverResolver for SqlxManeuverResolver {
    async fn get(&self, id: ManeuverId) -> Result<Option<Maneuver>, TransactionError> {
        if let Some(cached) = self.cache.get(&id).await {
            return Ok(Some(cached));
        }

        let mut maneuver_uow = self.maneuver_uow.clone();
        let mut tx = maneuver_uow.begin().await?;

        let maneuver = match tx.get_by_id(id).await {
            Ok(maneuver) => maneuver,
            Err(err) => {
                return match tx.rollback().await {
                    Ok(()) => Err(err),
                    Err(rollback_err) => Err(rollback_err),
                };
            }
        };

        tx.commit().await?;

        match maneuver {
            None => Ok(None),
            Some(maneuver) => {
                self.cache.insert(id, maneuver.clone()).await;
                Ok(Some(maneuver))
            }
        }
    }
}
