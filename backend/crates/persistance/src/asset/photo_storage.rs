use std::io::Cursor;
use std::path::PathBuf;

use image::imageops::FilterType;
use image::{DynamicImage, ImageFormat};
use rc_log_domain::asset::name::AssetName;
use rc_log_domain::asset::path::AssetPath;
use rc_log_domain::asset::photo::{Photo, PhotoId};
use rc_log_domain::asset::photo_storage::{PhotoStorage, PhotoStorageError};
use sqlx::PgPool;
use tokio::fs;
use uuid::Uuid;

/// Encode a `DynamicImage` to WebP bytes.
fn encode_webp(img: &DynamicImage) -> Result<Vec<u8>, PhotoStorageError> {
    let mut buf = Cursor::new(Vec::new());
    img.write_to(&mut buf, ImageFormat::WebP)
        .map_err(|e| PhotoStorageError::InvalidData(format!("WebP encode error: {e}")))?;
    Ok(buf.into_inner())
}

/// Resize `img` so its longest side is at most `max_px`, preserving aspect ratio.
/// Uses Lanczos3 for high quality.
fn resize_to(img: &DynamicImage, max_px: u32) -> DynamicImage {
    let (w, h) = (img.width(), img.height());
    let longest = w.max(h);
    if longest <= max_px {
        return img.clone();
    }
    let scale = max_px as f64 / longest as f64;
    let new_w = (w as f64 * scale).round() as u32;
    let new_h = (h as f64 * scale).round() as u32;
    img.resize(new_w, new_h, FilterType::Lanczos3)
}

/// Resolve a file path relative to `asset_path` and convert to a domain `AssetPath`.
fn make_asset_path(
    asset_path: &std::path::Path,
    rel: &str,
) -> Result<AssetPath, PhotoStorageError> {
    let _ = asset_path; // absolute path only used for writing; stored path is relative
    AssetPath::new(rel.to_string())
        .map_err(|e| PhotoStorageError::InvalidData(format!("Invalid asset path: {e}")))
}

#[derive(Clone)]
pub struct DiskDbPhotoStorage {
    pool: PgPool,
    asset_path: PathBuf,
}

impl DiskDbPhotoStorage {
    pub fn new(pool: PgPool, asset_path: PathBuf) -> Self {
        Self { pool, asset_path }
    }
}

impl PhotoStorage for DiskDbPhotoStorage {
    async fn store(&self, name: &AssetName, data: &[u8]) -> Result<Photo, PhotoStorageError> {
        let data = data.to_vec();
        let name_str = name.as_str().to_string();
        let asset_path = self.asset_path.clone();

        // All CPU-bound work (decode, resize, encode) runs off the async executor.
        let (small_bytes, medium_bytes, large_bytes) =
            tokio::task::spawn_blocking(move || -> Result<_, PhotoStorageError> {
                let img = image::load_from_memory(&data)
                    .map_err(|e| PhotoStorageError::InvalidData(format!("Decode error: {e}")))?;

                let longest = img.width().max(img.height());

                let small = resize_to(&img, 400);
                let small_bytes = encode_webp(&small)?;

                let medium_bytes = if longest > 400 {
                    let medium = resize_to(&img, 800);
                    Some(encode_webp(&medium)?)
                } else {
                    None
                };

                let large_bytes = if longest > 800 {
                    Some(encode_webp(&img)?)
                } else {
                    None
                };

                // Verify paths are constructable (side-effect free check).
                let photos_dir = asset_path.join("photos");
                let _ = photos_dir; // used below in async context

                Ok((small_bytes, medium_bytes, large_bytes))
            })
            .await
            .map_err(|e| PhotoStorageError::IoError(format!("spawn_blocking error: {e}")))??;

        let photos_dir = self.asset_path.join("photos");
        fs::create_dir_all(&photos_dir)
            .await
            .map_err(|e| PhotoStorageError::IoError(format!("create_dir_all: {e}")))?;

        let small_rel = format!("photos/{}_small.webp", name_str);
        let small_abs = self.asset_path.join(&small_rel);
        fs::write(&small_abs, &small_bytes)
            .await
            .map_err(|e| PhotoStorageError::IoError(format!("write small: {e}")))?;

        let medium_rel = medium_bytes
            .as_ref()
            .map(|_| format!("photos/{}_medium.webp", name_str));
        if let (Some(rel), Some(bytes)) = (medium_rel.as_ref(), medium_bytes.as_ref()) {
            let abs = self.asset_path.join(rel);
            fs::write(&abs, bytes)
                .await
                .map_err(|e| PhotoStorageError::IoError(format!("write medium: {e}")))?;
        }

        let large_rel = large_bytes
            .as_ref()
            .map(|_| format!("photos/{}_large.webp", name_str));
        if let (Some(rel), Some(bytes)) = (large_rel.as_ref(), large_bytes.as_ref()) {
            let abs = self.asset_path.join(rel);
            fs::write(&abs, bytes)
                .await
                .map_err(|e| PhotoStorageError::IoError(format!("write large: {e}")))?;
        }

        let small_path = make_asset_path(&self.asset_path, &small_rel)?;
        let medium_path = medium_rel
            .map(|r| make_asset_path(&self.asset_path, &r))
            .transpose()?;
        let large_path = large_rel
            .map(|r| make_asset_path(&self.asset_path, &r))
            .transpose()?;

        let photo_id = PhotoId::new(Uuid::new_v4());
        sqlx::query(
            r#"
            INSERT INTO asset.photo (id, name, small_path, medium_path, large_path)
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT (name) DO UPDATE
                SET small_path  = EXCLUDED.small_path,
                    medium_path = EXCLUDED.medium_path,
                    large_path  = EXCLUDED.large_path
            "#,
        )
        .bind(Uuid::from(photo_id))
        .bind(name.as_str())
        .bind(small_path.as_str())
        .bind(medium_path.as_ref().map(|p| p.as_str()))
        .bind(large_path.as_ref().map(|p| p.as_str()))
        .execute(&self.pool)
        .await
        .map_err(|e| PhotoStorageError::DatabaseError(format!("upsert photo: {e}")))?;

        Ok(Photo::new(photo_id, name.clone(), small_path, medium_path, large_path))
    }

    async fn delete(&self, name: &AssetName) -> Result<(), PhotoStorageError> {
        #[derive(sqlx::FromRow)]
        struct PathsRow {
            small_path: String,
            medium_path: Option<String>,
            large_path: Option<String>,
        }

        let row: Option<PathsRow> = sqlx::query_as(
            r#"SELECT small_path, medium_path, large_path FROM asset.photo WHERE name = $1"#,
        )
        .bind(name.as_str())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| PhotoStorageError::DatabaseError(format!("fetch paths: {e}")))?;

        sqlx::query(r#"DELETE FROM asset.photo WHERE name = $1"#)
            .bind(name.as_str())
            .execute(&self.pool)
            .await
            .map_err(|e| PhotoStorageError::DatabaseError(format!("delete photo: {e}")))?;

        if let Some(row) = row {
            for rel_path in [Some(row.small_path), row.medium_path, row.large_path]
                .into_iter()
                .flatten()
            {
                let abs = self.asset_path.join(&rel_path);
                if let Err(e) = fs::remove_file(&abs).await {
                    if e.kind() != std::io::ErrorKind::NotFound {
                        tracing::warn!(path = %abs.display(), error = %e, "Failed to delete photo file");
                    }
                }
            }
        }

        Ok(())
    }
}
