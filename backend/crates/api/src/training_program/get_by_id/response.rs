use rc_log_application::training_program::get_by_id::model::TrainingProgramDto;
use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetByIdResponse {
    #[serde(flatten)]
    pub data: TrainingProgramDto,
}

impl From<TrainingProgramDto> for GetByIdResponse {
    fn from(dto: TrainingProgramDto) -> Self {
        Self { data: dto }
    }
}
