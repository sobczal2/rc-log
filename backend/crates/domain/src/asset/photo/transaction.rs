use std::future::Future;

use crate::asset::name::Name;
use crate::asset::photo::Photo;
use crate::shared::transaction::{Transaction, TransactionError};

pub trait PhotoTransaction: Transaction<Photo> {
    fn get_by_name(
        &mut self,
        name: &Name,
    ) -> impl Future<Output = Result<Option<Photo>, TransactionError>> + Send;

    fn delete_by_name(
        &mut self,
        name: &Name,
    ) -> impl Future<Output = Result<(), TransactionError>> + Send;
}
