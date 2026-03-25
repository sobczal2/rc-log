use uuid::Uuid;

use crate::maneuver::Maneuver;


pub trait ManeuverRepository: Send + Sync {
    fn get_by_id(&self, id: Uuid) -> impl Future<Output = Option<Maneuver>> + Send;
}
