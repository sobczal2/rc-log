use rc_log_application::session::update::model::SessionDto;
use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateSessionResponse {
    #[serde(flatten)]
    pub data: SessionDto,
}

impl From<SessionDto> for UpdateSessionResponse {
    fn from(dto: SessionDto) -> Self {
        Self { data: dto }
    }
}
