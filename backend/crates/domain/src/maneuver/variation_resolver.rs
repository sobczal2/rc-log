use std::future::Future;

use crate::maneuver::variation::{Variation, VariationId};
use crate::shared::transaction::TransactionError;

pub trait VariationResolver: Send + Sync {
    fn get(
        &self,
        variation_id: VariationId,
    ) -> impl Future<Output = Result<Option<Variation>, TransactionError>> + Send;
}