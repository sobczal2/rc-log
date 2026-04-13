use std::path::PathBuf;

use rc_log_persistance::asset::photo_resolver::SqlxPhotoResolver;
use rc_log_persistance::asset::photo_service::DiskDbPhotoService;
use rc_log_persistance::asset::video_resolver::SqlxVideoResolver;
use rc_log_persistance::maneuver::transaction::SqlxManeuverUnitOfWork;
use rc_log_persistance::model::transaction::SqlxModelUnitOfWork;
use rc_log_persistance::session::transaction::SqlxSessionUnitOfWork;
use rc_log_persistance::shared::cache_settings::CacheSettings;
use rc_log_persistance::user::transaction::SqlxUserUnitOfWork;
use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub maneuver_uow: SqlxManeuverUnitOfWork,
    pub model_uow: SqlxModelUnitOfWork,
    pub session_uow: SqlxSessionUnitOfWork,
    pub user_uow: SqlxUserUnitOfWork,
    pub video_resolver: SqlxVideoResolver,
    pub photo_resolver: SqlxPhotoResolver,
    pub photo_service: DiskDbPhotoService,
    pub jwt_secret: String,
}

impl AppState {
    pub fn new(
        pool: PgPool,
        jwt_secret: String,
        asset_cache_size: u64,
        asset_path: PathBuf,
    ) -> Self {
        let cache_settings = CacheSettings { capacity: asset_cache_size, ..CacheSettings::default() };

        Self {
            maneuver_uow: SqlxManeuverUnitOfWork::new(pool.clone()),
            model_uow: SqlxModelUnitOfWork::new(pool.clone()),
            session_uow: SqlxSessionUnitOfWork::new(pool.clone()),
            user_uow: SqlxUserUnitOfWork::new(pool.clone()),
            video_resolver: SqlxVideoResolver::new(pool.clone(), cache_settings.clone()),
            photo_resolver: SqlxPhotoResolver::new(pool.clone(), cache_settings),
            photo_service: DiskDbPhotoService::new(pool, asset_path),
            jwt_secret,
        }
    }
}
