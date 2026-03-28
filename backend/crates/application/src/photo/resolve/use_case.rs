use rc_log_domain::asset::name::AssetName;
use rc_log_domain::asset::photo_resolver::PhotoResolver;
use tracing::{debug, instrument};

use crate::error::ApplicationError;

use super::error::ResolvePhotoError;
use super::model::{PhotoPathsDto, ResolvePhotoInput};

pub struct ResolvePhotoUseCase<R> {
    resolver: R,
}

impl<R: PhotoResolver> ResolvePhotoUseCase<R> {
    pub fn new(resolver: R) -> Self {
        Self { resolver }
    }

    #[instrument(skip(self), fields(name = %input.name))]
    pub async fn execute(
        &self,
        input: ResolvePhotoInput,
    ) -> Result<PhotoPathsDto, ApplicationError> {
        debug!("Resolving photo asset paths");

        let name = AssetName::new(input.name.clone())
            .map_err(|e| ResolvePhotoError::InvalidName(e.to_string()))?;

        let photo = self
            .resolver
            .get(&name)
            .await
            .map_err(ResolvePhotoError::from)?
            .ok_or(ResolvePhotoError::NotFound)?;

        debug!("Photo asset resolved");
        Ok(PhotoPathsDto::from(photo))
    }
}
