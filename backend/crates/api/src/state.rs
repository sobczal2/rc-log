use rc_log_persistance::maneuver::repository::SqlxManeuverUnitOfWork;
use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub maneuver_uow: SqlxManeuverUnitOfWork,
}

impl AppState {
    pub fn new(pool: PgPool) -> Self {
        Self {
            maneuver_uow: SqlxManeuverUnitOfWork::new(pool),
        }
    }
}
