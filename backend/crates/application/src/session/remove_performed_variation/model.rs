use rc_log_domain::session::Session;
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct RemovePerformedVariationInput {
    pub session_id: Uuid,
    pub owner_id: Uuid,
    pub variation_id: Uuid,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionDto {
    pub id: Uuid,
    pub user_id: Uuid,
    pub date: String,
    pub model_id: Option<Uuid>,
    pub note: Option<String>,
}

impl From<Session> for SessionDto {
    fn from(session: Session) -> Self {
        Self {
            id: Uuid::from(session.id()),
            user_id: Uuid::from(session.user_id()),
            date: session.date().as_naive_date().format("%Y-%m-%d").to_string(),
            model_id: session.model_id().map(Uuid::from),
            note: session.note().map(|n| n.as_str().to_string()),
        }
    }
}
