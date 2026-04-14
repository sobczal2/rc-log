use std::future::Future;

use crate::asset::video::{Video, VideoId};
use crate::shared::transaction::TransactionError;

pub trait VideoResolver: Send + Sync {
    fn get(
        &self,
        id: &VideoId,
    ) -> impl Future<Output = Result<Option<Video>, TransactionError>> + Send;
}
