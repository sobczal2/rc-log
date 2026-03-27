use std::future::Future;

use super::name::AssetName;
use super::video::Video;
use crate::shared::transaction::TransactionError;

pub trait VideoResolver: Send + Sync {
    fn get(
        &self,
        name: &AssetName,
    ) -> impl Future<Output = Result<Option<Video>, TransactionError>> + Send;
}
