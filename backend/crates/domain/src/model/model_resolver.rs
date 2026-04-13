use std::future::Future;

use crate::model::Model;
use crate::model::id::ModelId;
use crate::shared::transaction::TransactionError;

pub trait ModelResolver: Send + Sync {
    fn get_by_id(
        &self,
        id: &ModelId,
    ) -> impl Future<Output = Result<Option<Model>, TransactionError>> + Send;
}
