use rc_log_domain::asset::name::AssetName;
use rc_log_domain::asset::video_resolver::VideoResolver;
use tracing::{debug, instrument};

use crate::error::ApplicationError;

use super::error::ResolveVideoError;
use super::model::{ResolveVideoInput, VideoPathsDto};

pub struct ResolveVideoUseCase<R> {
    resolver: R,
}

impl<R: VideoResolver> ResolveVideoUseCase<R> {
    pub fn new(resolver: R) -> Self {
        Self { resolver }
    }

    #[instrument(skip(self), fields(name = %input.name))]
    pub async fn execute(
        &self,
        input: ResolveVideoInput,
    ) -> Result<VideoPathsDto, ApplicationError> {
        debug!("Resolving video asset paths");

        let name = AssetName::new(input.name.clone())
            .map_err(|e| ResolveVideoError::InvalidName(e.to_string()))?;

        let video = self
            .resolver
            .get(&name)
            .await
            .map_err(ResolveVideoError::from)?
            .ok_or(ResolveVideoError::NotFound)?;

        debug!("Video asset resolved");
        Ok(VideoPathsDto::from(video))
    }
}
