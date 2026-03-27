use rc_log_application::video::resolve::model::VideoPathsDto;
use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResolveVideoResponse {
    #[serde(flatten)]
    pub data: VideoPathsDto,
}

impl From<VideoPathsDto> for ResolveVideoResponse {
    fn from(dto: VideoPathsDto) -> Self {
        Self { data: dto }
    }
}
