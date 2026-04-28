use std::future::Future;

use crate::photo::{Photo, PhotoId};
use crate::shared::resolver::ResolverError;

pub trait PhotoResolver: Send + Sync {
    fn get(
        &self,
        id: PhotoId,
    ) -> impl Future<Output = Result<Option<Photo>, ResolverError>> + Send;
}
