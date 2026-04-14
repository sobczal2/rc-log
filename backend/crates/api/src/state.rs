use std::path::PathBuf;

use rc_log_persistance::asset::photo::resolver::SqlxPhotoResolver;
use rc_log_persistance::asset::photo::service::DiskDbPhotoService;
use rc_log_persistance::asset::video::resolver::SqlxVideoResolver;
use rc_log_persistance::maneuver::resolver::SqlxManeuverResolver;
use rc_log_persistance::maneuver::transaction::SqlxManeuverUnitOfWork;
use rc_log_persistance::maneuver::variation::resolver::SqlxVariationResolver;
use rc_log_persistance::model::resolver::SqlxModelResolver;
use rc_log_persistance::model::transaction::SqlxModelUnitOfWork;
use rc_log_persistance::session::transaction::SqlxSessionUnitOfWork;
use rc_log_persistance::shared::cache_settings::CacheSettings;
use rc_log_persistance::user::transaction::SqlxUserUnitOfWork;
use sqlx::PgPool;
use std::time::Duration;

#[derive(Clone, Copy)]
pub struct ResolverCacheConfig {
    pub model_ttl_seconds: u64,
    pub model_size: u64,
    pub maneuver_ttl_seconds: u64,
    pub maneuver_size: u64,
    pub variation_ttl_seconds: u64,
    pub variation_size: u64,
    pub video_ttl_seconds: u64,
    pub video_size: u64,
    pub photo_ttl_seconds: u64,
    pub photo_size: u64,
}

#[derive(Clone)]
pub struct AppState {
    pub maneuver_uow: SqlxManeuverUnitOfWork,
    pub model_uow: SqlxModelUnitOfWork,
    pub session_uow: SqlxSessionUnitOfWork,
    pub user_uow: SqlxUserUnitOfWork,
    pub model_resolver: SqlxModelResolver,
    pub maneuver_resolver: SqlxManeuverResolver,
    pub variation_resolver: SqlxVariationResolver,
    pub video_resolver: SqlxVideoResolver,
    pub photo_resolver: SqlxPhotoResolver,
    pub photo_service: DiskDbPhotoService,
    pub jwt_secret: String,
}

impl AppState {
    pub fn new(pool: PgPool, jwt_secret: String, caches: ResolverCacheConfig, asset_path: PathBuf) -> Self {
        let model_cache_settings = CacheSettings {
            capacity: caches.model_size,
            ttl: Duration::from_secs(caches.model_ttl_seconds),
        };
        let maneuver_cache_settings = CacheSettings {
            capacity: caches.maneuver_size,
            ttl: Duration::from_secs(caches.maneuver_ttl_seconds),
        };
        let variation_cache_settings = CacheSettings {
            capacity: caches.variation_size,
            ttl: Duration::from_secs(caches.variation_ttl_seconds),
        };
        let video_cache_settings = CacheSettings {
            capacity: caches.video_size,
            ttl: Duration::from_secs(caches.video_ttl_seconds),
        };
        let photo_cache_settings = CacheSettings {
            capacity: caches.photo_size,
            ttl: Duration::from_secs(caches.photo_ttl_seconds),
        };

        Self {
            maneuver_uow: SqlxManeuverUnitOfWork::new(pool.clone()),
            model_uow: SqlxModelUnitOfWork::new(pool.clone()),
            session_uow: SqlxSessionUnitOfWork::new(pool.clone()),
            user_uow: SqlxUserUnitOfWork::new(pool.clone()),
            model_resolver: SqlxModelResolver::new(pool.clone(), model_cache_settings),
            maneuver_resolver: SqlxManeuverResolver::new(pool.clone(), maneuver_cache_settings),
            variation_resolver: SqlxVariationResolver::new(pool.clone(), variation_cache_settings),
            video_resolver: SqlxVideoResolver::new(pool.clone(), video_cache_settings),
            photo_resolver: SqlxPhotoResolver::new(pool.clone(), photo_cache_settings),
            photo_service: DiskDbPhotoService::new(pool, asset_path),
            jwt_secret,
        }
    }
}
