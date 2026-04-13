use axum::{Json, extract::State};
use axum::extract::multipart::Multipart;
use axum::extract::Path;
use rc_log_application::model::update_photo::UpdateModelPhotoUseCase;
use rc_log_application::model::update_photo::model::UpdateModelPhotoInput;
use rc_log_application::shared::validator::ValidationError;
use tracing::{debug, instrument};
use uuid::Uuid;

use crate::error::ApiError;
use crate::extractors::auth::AuthenticatedUser;
use crate::model::update_photo::response::UpdatePhotoResponse;
use crate::state::AppState;

#[instrument(skip(state, multipart), fields(model_id = %model_id, owner_id = %auth.id))]
pub async fn update_model_photo(
    State(state): State<AppState>,
    auth: AuthenticatedUser,
    Path(model_id): Path<Uuid>,
    mut multipart: Multipart,
) -> Result<Json<UpdatePhotoResponse>, ApiError> {
    debug!("Handling update_model_photo request");

    let mut photo_bytes: Option<Vec<u8>> = None;

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| ApiError::Validation(vec![ValidationError::new("multipart", e.to_string())]))?
    {
        if field.name() == Some("photo") {
            let content_type = field.content_type().unwrap_or("").to_string();
            if !matches!(
                content_type.as_str(),
                "image/jpeg" | "image/png" | "image/webp"
            ) {
                return Err(ApiError::Validation(vec![ValidationError::new(
                    "photo",
                    "content-type must be image/jpeg, image/png, or image/webp",
                )]));
            }

            let bytes = field.bytes().await.map_err(|e| {
                ApiError::Validation(vec![ValidationError::new("photo", e.to_string())])
            })?;
            photo_bytes = Some(bytes.to_vec());
            break;
        }
    }

    let data = photo_bytes.ok_or_else(|| {
        ApiError::Validation(vec![ValidationError::new(
            "photo",
            "field 'photo' is required",
        )])
    })?;

    let mut use_case = UpdateModelPhotoUseCase::new(state.model_uow, state.photo_storage);
    let dto = use_case
        .execute(UpdateModelPhotoInput {
            model_id,
            owner_id: auth.id,
            data,
        })
        .await?;

    debug!("Model photo updated");
    Ok(Json(UpdatePhotoResponse::from(dto)))
}
