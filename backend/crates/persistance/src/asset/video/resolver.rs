use crate::shared::cache_settings::CacheSettings;
use moka::future::Cache;
use rc_log_domain::asset::video::resolver::VideoResolver;
use rc_log_domain::asset::video::transaction::VideoTransaction;
use rc_log_domain::asset::video::{Video, VideoId};
use rc_log_domain::shared::resolver::ResolverError;
use rc_log_domain::shared::transaction::Transaction;
use rc_log_domain::shared::transaction::TransactionError;
use rc_log_domain::shared::unit_of_work::UnitOfWork;
use sqlx::PgPool;

use super::transaction::SqlxVideoUnitOfWork;

fn tx_err_to_resolver(e: TransactionError) -> ResolverError {
    match e {
        TransactionError::InvalidData(msg) => ResolverError::InvalidData(msg),
        TransactionError::TransactionError(msg) => ResolverError::ResolverError(msg),
    }
}

#[derive(Clone)]
pub struct SqlxVideoResolver {
    video_uow: SqlxVideoUnitOfWork,
    cache: Cache<String, Video>,
}

impl SqlxVideoResolver {
    pub fn new(pool: PgPool, settings: CacheSettings) -> Self {
        let cache =
            Cache::builder().max_capacity(settings.capacity).time_to_live(settings.ttl).build();
        Self { video_uow: SqlxVideoUnitOfWork::new(pool), cache }
    }
}

impl VideoResolver for SqlxVideoResolver {
    async fn get(&self, id: VideoId) -> Result<Option<Video>, ResolverError> {
        let key = id.as_uuid().to_string();

        if let Some(cached) = self.cache.get(&key).await {
            return Ok(Some(cached));
        }

        let mut video_uow = self.video_uow.clone();
        let mut tx = video_uow.begin().await.map_err(tx_err_to_resolver)?;

        let video = match tx.get_by_id(&id).await {
            Ok(video) => video,
            Err(err) => {
                let _ = tx.rollback().await;
                return Err(tx_err_to_resolver(err));
            }
        };

        tx.commit().await.map_err(tx_err_to_resolver)?;

        match video {
            None => Ok(None),
            Some(video) => {
                self.cache.insert(key, video.clone()).await;
                Ok(Some(video))
            }
        }
    }
}
