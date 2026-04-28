use std::future::Future;

use crate::video::{Video, VideoId};
use crate::shared::transaction::{Transaction, TransactionError};

pub trait VideoTransaction: Transaction<Video> {
    fn get_by_id(
        &mut self,
        id: &VideoId,
    ) -> impl Future<Output = Result<Option<Video>, TransactionError>> + Send;

    fn delete_by_id(
        &mut self,
        id: &VideoId,
    ) -> impl Future<Output = Result<(), TransactionError>> + Send;
}
