use std::future::Future;

use crate::asset::name::Name;
use crate::asset::video::Video;
use crate::shared::transaction::TransactionError;

pub trait VideoResolver: Send + Sync {
    fn get(
        &self,
        name: &Name,
    ) -> impl Future<Output = Result<Option<Video>, TransactionError>> + Send;
}
