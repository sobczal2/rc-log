use uuid::Uuid;

#[derive(Debug)]
pub struct RemoveUserPhotoInput {
    pub user_id: Uuid,
}
