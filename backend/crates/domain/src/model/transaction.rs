use std::future::Future;

use crate::model::Model;
use crate::model::id::ModelId;
use crate::shared::pagination::Pagination;
use crate::shared::transaction::{Transaction, TransactionError};
use crate::user::id::UserId;

pub trait ModelTransaction: Transaction<Model> {
    fn get_by_id(
        &mut self,
        id: ModelId,
    ) -> impl Future<Output = Result<Option<Model>, TransactionError>> + Send;

    fn list_by_owner(
        &mut self,
        owner_id: UserId,
        pagination: Pagination,
    ) -> impl Future<Output = Result<(Vec<Model>, u64), TransactionError>> + Send;
}
