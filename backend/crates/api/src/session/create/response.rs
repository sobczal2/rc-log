use rc_log_application::session::create::model::SessionDto;
use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateSessionResponse {
    #[serde(flatten)]
    pub data: SessionDto,
}

impl From<SessionDto> for CreateSessionResponse {
    fn from(dto: SessionDto) -> Self {
        Self { data: dto }
    }
}
