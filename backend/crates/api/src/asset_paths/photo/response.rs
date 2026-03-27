use rc_log_application::photo::resolve::model::PhotoPathsDto;
use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResolvePhotoResponse {
    #[serde(flatten)]
    pub data: PhotoPathsDto,
}

impl From<PhotoPathsDto> for ResolvePhotoResponse {
    fn from(dto: PhotoPathsDto) -> Self {
        Self { data: dto }
    }
}
