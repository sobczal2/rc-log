use crate::cache::store::Cache;
use rc_log_domain::maneuver::Maneuver;
use rc_log_domain::maneuver::id::ManeuverId;
use rc_log_domain::maneuver::resolver::ManeuverResolver;
use rc_log_domain::maneuver::transaction::ManeuverTransaction;
use rc_log_domain::shared::resolver::ResolverError;
use rc_log_domain::shared::transaction::Transaction;
use rc_log_domain::shared::transaction::TransactionError;
use rc_log_domain::shared::unit_of_work::UnitOfWork;
use sqlx::PgPool;

use super::transaction::SqlxManeuverUnitOfWork;

fn tx_err_to_resolver(e: TransactionError) -> ResolverError {
    match e {
        TransactionError::InvalidData(msg) => ResolverError::InvalidData(msg),
        TransactionError::TransactionError(msg) => ResolverError::ResolverError(msg),
    }
}

#[derive(Clone)]
pub struct SqlxManeuverResolver {
    maneuver_uow: SqlxManeuverUnitOfWork,
    cache: Cache<ManeuverId, Maneuver>,
}

impl SqlxManeuverResolver {
    pub fn new(pool: PgPool, cache: Cache<ManeuverId, Maneuver>) -> Self {
        Self { maneuver_uow: SqlxManeuverUnitOfWork::new(pool), cache }
    }
}

impl ManeuverResolver for SqlxManeuverResolver {
    async fn get(&self, id: ManeuverId) -> Result<Option<Maneuver>, ResolverError> {
        if let Some(cached) = self.cache.get(id.clone()).await {
            return Ok(Some(cached));
        }

        let mut maneuver_uow = self.maneuver_uow.clone();
        let mut tx = maneuver_uow.begin().await.map_err(tx_err_to_resolver)?;

        let maneuver = match tx.get_by_id(id).await {
            Ok(maneuver) => maneuver,
            Err(err) => {
                let _ = tx.rollback().await;
                return Err(tx_err_to_resolver(err));
            }
        };

        tx.commit().await.map_err(tx_err_to_resolver)?;

        match maneuver {
            None => Ok(None),
            Some(maneuver) => {
                self.cache.insert(id, maneuver.clone()).await;
                Ok(Some(maneuver))
            }
        }
    }
}
