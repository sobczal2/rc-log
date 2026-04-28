use std::future::Future;

use crate::model::Model;
use crate::model::id::ModelId;
use crate::shared::resolver::ResolverError;

pub trait ModelResolver: Send + Sync {
    fn get(
        &self,
        id: ModelId,
    ) -> impl Future<Output = Result<Option<Model>, ResolverError>> + Send;
}
