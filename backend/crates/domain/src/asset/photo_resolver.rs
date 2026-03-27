use std::future::Future;

use super::name::AssetName;
use super::photo::Photo;
use crate::shared::transaction::TransactionError;

pub trait PhotoResolver: Send + Sync {
    fn get(
        &self,
        name: &AssetName,
    ) -> impl Future<Output = Result<Option<Photo>, TransactionError>> + Send;
}
