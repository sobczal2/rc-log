use rc_log_domain::asset::photo::PhotoId;
use rc_log_domain::asset::photo::resolver::PhotoResolver;
use tracing::{debug, instrument};
use uuid::Uuid;

use super::error::ResolvePhotoError;
use super::model::{PhotoPathsDto, ResolvePhotoInput};

pub struct ResolvePhotoUseCase<R> {
    resolver: R,
}

impl<R: PhotoResolver> ResolvePhotoUseCase<R> {
    pub fn new(resolver: R) -> Self {
        Self { resolver }
    }

    #[instrument(skip(self), fields(photo_id = %input.id))]
    pub async fn execute(
        &self,
        input: ResolvePhotoInput,
    ) -> Result<PhotoPathsDto, ResolvePhotoError> {
        debug!("Resolving photo asset paths");

        let id =
            Uuid::parse_str(&input.id).map_err(|_| ResolvePhotoError::InvalidId { id: input.id.clone() })?;
        let photo_id = PhotoId::new(id);

        let photo = self
            .resolver
            .get(photo_id)
            .await
            .map_err(ResolvePhotoError::from)?
            .ok_or(ResolvePhotoError::NotFound)?;

        debug!("Photo asset resolved");
        Ok(PhotoPathsDto::from(photo))
    }
}
