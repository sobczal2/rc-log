use axum::extract::multipart::Multipart;
use axum::extract::{FromRequest, FromRequestParts, Path};
use rc_log_application::shared::validator::ValidationError;
use uuid::Uuid;

use crate::error::ApiError;

pub struct UpdatePhotoRequest {
    pub model_id: Uuid,
    pub data: Vec<u8>,
}

impl<S> FromRequest<S> for UpdatePhotoRequest
where
    S: Send + Sync,
{
    type Rejection = ApiError;

    async fn from_request(req: axum::extract::Request, state: &S) -> Result<Self, Self::Rejection> {
        let mut parts = req.into_parts();

        let model_id = {
            let Path(id) = Path::<Uuid>::from_request_parts(&mut parts.0, state)
                .await
                .map_err(|e| {
                    ApiError::Validation(vec![ValidationError::new("id", e.to_string())])
                })?;
            if id.is_nil() {
                return Err(ApiError::Validation(vec![ValidationError::new(
                    "id",
                    "must not be nil",
                )]));
            }
            id
        };

        let reconstructed = axum::extract::Request::from_parts(parts.0, parts.1);
        let mut multipart = Multipart::from_request(reconstructed, state)
            .await
            .map_err(|e| {
                ApiError::Validation(vec![ValidationError::new("multipart", e.to_string())])
            })?;

        let mut photo_bytes: Option<Vec<u8>> = None;

        while let Some(field) = multipart.next_field().await.map_err(|e| {
            ApiError::Validation(vec![ValidationError::new("multipart", e.to_string())])
        })? {
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

        Ok(Self { model_id, data })
    }
}
