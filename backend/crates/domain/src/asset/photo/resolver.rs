use std::future::Future;

use crate::asset::name::Name;
use crate::asset::photo::Photo;
use crate::shared::transaction::TransactionError;

pub trait PhotoResolver: Send + Sync {
    fn get(
        &self,
        name: &Name,
    ) -> impl Future<Output = Result<Option<Photo>, TransactionError>> + Send;
}
