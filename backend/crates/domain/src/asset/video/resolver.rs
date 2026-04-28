use std::future::Future;

use crate::asset::video::{Video, VideoId};
use crate::shared::resolver::ResolverError;

pub trait VideoResolver: Send + Sync {
    fn get(
        &self,
        id: VideoId,
    ) -> impl Future<Output = Result<Option<Video>, ResolverError>> + Send;
}
