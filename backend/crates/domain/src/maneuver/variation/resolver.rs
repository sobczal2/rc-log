use std::future::Future;

use crate::maneuver::variation::{Variation, VariationId};
use crate::shared::resolver::ResolverError;

pub trait VariationResolver: Send + Sync {
    fn get(
        &self,
        variation_id: VariationId,
    ) -> impl Future<Output = Result<Option<Variation>, ResolverError>> + Send;
}
