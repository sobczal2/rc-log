use std::future::Future;

use crate::asset::photo::{Photo, PhotoId};
use crate::shared::transaction::{Transaction, TransactionError};

pub trait PhotoTransaction: Transaction<Photo> {
    fn get_by_id(
        &mut self,
        id: &PhotoId,
    ) -> impl Future<Output = Result<Option<Photo>, TransactionError>> + Send;

    fn delete_by_id(
        &mut self,
        id: &PhotoId,
    ) -> impl Future<Output = Result<(), TransactionError>> + Send;
}
