use crate::shared::cache_settings::CacheSettings;
use moka::future::Cache;
use rc_log_domain::asset::name::Name;
use rc_log_domain::asset::video::Video;
use rc_log_domain::asset::video::resolver::VideoResolver;
use rc_log_domain::asset::video::transaction::VideoTransaction;
use rc_log_domain::shared::transaction::Transaction;
use rc_log_domain::shared::transaction::TransactionError;
use rc_log_domain::shared::unit_of_work::UnitOfWork;
use sqlx::PgPool;

use super::video_transaction::SqlxVideoUnitOfWork;

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
    async fn get(&self, name: &Name) -> Result<Option<Video>, TransactionError> {
        let key = name.as_str().to_string();

        if let Some(cached) = self.cache.get(&key).await {
            return Ok(Some(cached));
        }

        let mut video_uow = self.video_uow.clone();
        let mut tx = video_uow.begin().await?;

        let video = match tx.get_by_name(name).await {
            Ok(video) => video,
            Err(err) => {
                return match tx.rollback().await {
                    Ok(()) => Err(err),
                    Err(rollback_err) => Err(rollback_err),
                };
            }
        };

        tx.commit().await?;

        match video {
            None => Ok(None),
            Some(video) => {
                self.cache.insert(key, video.clone()).await;
                Ok(Some(video))
            }
        }
    }
}
