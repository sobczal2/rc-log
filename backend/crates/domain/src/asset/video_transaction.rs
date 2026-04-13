use std::future::Future;

use crate::asset::name::AssetName;
use crate::asset::video::Video;
use crate::shared::transaction::{Transaction, TransactionError};

pub trait VideoTransaction: Transaction<Video> {
    fn get_by_name(
        &mut self,
        name: &AssetName,
    ) -> impl Future<Output = Result<Option<Video>, TransactionError>> + Send;

    fn delete_by_name(
        &mut self,
        name: &AssetName,
    ) -> impl Future<Output = Result<(), TransactionError>> + Send;
}
