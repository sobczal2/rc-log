use std::future::Future;

use crate::maneuver::Maneuver;
use crate::maneuver::id::ManeuverId;
use crate::shared::resolver::ResolverError;

pub trait ManeuverResolver: Send + Sync {
    fn get(
        &self,
        id: ManeuverId,
    ) -> impl Future<Output = Result<Option<Maneuver>, ResolverError>> + Send;
}
