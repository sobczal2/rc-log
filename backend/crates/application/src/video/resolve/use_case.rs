use rc_log_domain::asset::video::VideoId;
use rc_log_domain::asset::video::resolver::VideoResolver;
use tracing::{debug, instrument};
use uuid::Uuid;

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

    #[instrument(skip(self), fields(video_id = %input.id))]
    pub async fn execute(
        &self,
        input: ResolveVideoInput,
    ) -> Result<VideoPathsDto, ApplicationError> {
        debug!("Resolving video asset paths");

        let id =
            Uuid::parse_str(&input.id).map_err(|e| ResolveVideoError::InvalidId(e.to_string()))?;
        let video_id = VideoId::new(id);

        let video = self
            .resolver
            .get(&video_id)
            .await
            .map_err(ResolveVideoError::from)?
            .ok_or(ResolveVideoError::NotFound)?;

        debug!("Video asset resolved");
        Ok(VideoPathsDto::from(video))
    }
}
