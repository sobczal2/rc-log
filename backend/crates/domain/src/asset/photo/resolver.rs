use std::future::Future;

use crate::asset::photo::{Photo, PhotoId};
use crate::shared::transaction::TransactionError;

pub trait PhotoResolver: Send + Sync {
    fn get(
        &self,
        id: &PhotoId,
    ) -> impl Future<Output = Result<Option<Photo>, TransactionError>> + Send;
}
