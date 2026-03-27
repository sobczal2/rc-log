use rc_log_persistance::maneuver::transaction::SqlxManeuverUnitOfWork;
use rc_log_persistance::user::transaction::SqlxUserUnitOfWork;
use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub maneuver_uow: SqlxManeuverUnitOfWork,
    pub user_uow: SqlxUserUnitOfWork,
    pub jwt_secret: String,
}

impl AppState {
    pub fn new(pool: PgPool, jwt_secret: String) -> Self {
        Self {
            maneuver_uow: SqlxManeuverUnitOfWork::new(pool.clone()),
            user_uow: SqlxUserUnitOfWork::new(pool),
            jwt_secret,
        }
    }
}
